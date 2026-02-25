//! RISCVML Module 6: Object/Obstacle Detection
//!
//! Detect → Visualize → React pattern applied to object/obstacle
//! classification. Uses the same ESP32-P4 infrastructure as the
//! bird detection capstone but swaps the model and color map.
//!
//! Pipeline: Camera/LiDAR → ML Classification → SQLite color lookup
//!           → RGB LED (threat-level color) → Proximity alert / brake / log

#![no_std]
#![no_main]

mod detect_db;
mod led;

use embassy_executor::Spawner;
use esp_hal::prelude::*;

/// Object classification categories.
pub enum ObjectClass {
    Person,
    Vehicle,
    Animal,
    Bicycle,
    Unknown,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("RISCVML Object Detection — Detect → Visualize → React");

    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Reuses bird-detection infrastructure:
    // camera.rs, isp.rs, ppa.rs, inference.rs (with object model)
    let db = detect_db::open_or_create("/sdcard/riscvml_detect.db");
    let led = led::init(&peripherals);

    // TODO: Initialize camera + ISP (shared with bird detection)
    // TODO: Load object detection model (SSD-MobileNet or YOLO-Nano)
    // TODO: Run Detect → Visualize → React loop
    //   - Classify objects: Person (yellow), Vehicle (cyan), Animal (magenta)
    //   - Set RGB LED to threat-level color
    //   - Trigger proximity alert if object within threshold distance

    loop {
        embassy_time::Timer::after_millis(33).await; // ~30 FPS
    }
}
