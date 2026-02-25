//! RISCVML Capstone: Bird Detection Pipeline on ESP32-P4
//!
//! Embassy async runtime orchestrating the full pipeline:
//!   Phase 1: Camera → ISP → Display (30-60 FPS preview)
//!   Phase 2: PPA downscale → esp-dl inference → overlay + SQLite + RGB LED
//!   Phase 3: Servo tracking → H.264 recording → C6 companion → MQTT/RTSP

#![no_std]
#![no_main]

mod camera;
mod companion;
mod detect_db;
mod display;
mod encoder;
mod inference;
mod isp;
mod led;
mod network;
mod ppa;
mod servo;

use embassy_executor::Spawner;
use esp_hal::prelude::*;

/// Frame buffer type — zero-copy ownership passed between pipeline stages.
/// The Rust borrow checker enforces that only one stage holds a mutable
/// reference to a frame buffer at any time, preventing data races without
/// runtime overhead.
pub struct FrameBuffer {
    pub data: &'static mut [u8],
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

/// Detection result from esp-dl inference, passed to servo tracking,
/// SQLite logging, RGB LED indicator, and bounding box overlay.
pub struct Detection {
    pub species: heapless::String<32>,
    pub confidence: f32,
    pub bbox: BoundingBox,
    pub timestamp_ms: u64,
}

pub struct BoundingBox {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("RISCVML Bird Detection Pipeline — starting");

    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Phase 1: Initialize camera → ISP → display pipeline
    let cam = camera::init(&peripherals);
    let isp_pipeline = isp::init(&peripherals);
    let disp = display::init(&peripherals);

    // Phase 2: Initialize ML inference subsystem
    let ppa_ctx = ppa::init(&peripherals);
    let model = inference::load_model();
    let db = detect_db::open_or_create("/sdcard/riscvml_detect.db");
    let led_driver = led::init(&peripherals);

    // Phase 3: Initialize tracking and recording
    let servos = servo::init(&peripherals);
    let enc = encoder::init(&peripherals);
    let c6 = companion::init(&peripherals);

    // Spawn concurrent pipeline tasks — Embassy schedules them cooperatively
    spawner.spawn(camera_capture_task(cam, isp_pipeline, disp, ppa_ctx)).unwrap();
    spawner.spawn(inference_task(model, db, led_driver)).unwrap();
    spawner.spawn(tracking_task(servos)).unwrap();
    spawner.spawn(recording_task(enc, c6)).unwrap();

    defmt::info!("All pipeline tasks spawned — running");
}

#[embassy_executor::task]
async fn camera_capture_task(
    mut cam: camera::CsiCamera,
    mut isp: isp::IspPipeline,
    mut disp: display::DsiDisplay,
    mut ppa_ctx: ppa::PpaContext,
) {
    loop {
        // Capture frame — ownership moves from DMA to this scope
        let frame = cam.capture_frame().await;

        // ISP processing: white balance → auto-exposure → demosaicing
        let processed = isp.process(frame).await;

        // Fork: display gets the frame, PPA downscales a copy for inference
        let downscaled = ppa.downscale(&processed, inference::MODEL_INPUT_SIZE).await;
        disp.push_frame(processed).await;

        // Send downscaled frame to inference queue
        inference::FRAME_QUEUE.send(downscaled).await;
    }
}

#[embassy_executor::task]
async fn inference_task(
    model: inference::Model,
    db: detect_db::DetectDb,
    mut led: led::RgbLed,
) {
    loop {
        let frame = inference::FRAME_QUEUE.receive().await;
        let detections = model.detect(&frame).await;

        for det in &detections {
            // Log to SQLite
            db.insert_detection(det);

            // Look up species → RGB color mapping
            let color = db.get_led_color(&det.species);
            led.set_color(color);

            // Queue bounding box overlay
            ppa::OVERLAY_QUEUE.send(det.bbox).await;

            defmt::info!(
                "Detected: {} ({:.1}%) at [{},{} {}x{}]",
                det.species.as_str(),
                det.confidence * 100.0,
                det.bbox.x, det.bbox.y, det.bbox.w, det.bbox.h,
            );
        }
    }
}

#[embassy_executor::task]
async fn tracking_task(mut servos: servo::PanTiltServos) {
    loop {
        let bbox = ppa::OVERLAY_QUEUE.receive().await;
        servos.track(bbox).await;
    }
}

#[embassy_executor::task]
async fn recording_task(
    mut enc: encoder::H264Encoder,
    mut c6: companion::C6Companion,
) {
    loop {
        let encoded = enc.encode_next_frame().await;
        c6.stream_chunk(&encoded).await;
    }
}
