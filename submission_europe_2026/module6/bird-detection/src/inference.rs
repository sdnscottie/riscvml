//! esp-dl Inference — Quantized Object Detection (ESP-IDF FFI)
//!
//! Wraps Espressif's esp-dl / esp-detection library for on-device bird
//! detection. The model runs quantized INT8 inference accelerated by the
//! ESP32-P4's 128-bit RISC-V vector extensions (RVV). PPA-downscaled
//! frames are fed to the model, and detection results (species, confidence,
//! bounding box) are returned as Rust structs.

use esp_idf_sys as _;
use embassy_sync::channel::Channel;

/// Model input dimensions — PPA downscales camera frames to this size.
pub const MODEL_INPUT_SIZE: (u32, u32) = (320, 320);

/// Channel for downscaled frames from PPA → inference engine.
pub static FRAME_QUEUE: Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    crate::FrameBuffer,
    2,
> = Channel::new();

/// Loaded and initialized esp-dl model handle.
pub struct Model {
    _handle: *mut core::ffi::c_void,
    num_classes: usize,
    confidence_threshold: f32,
}

/// Load the quantized bird detection model from flash.
///
/// The model binary is embedded at compile time via `include_bytes!`
/// and uses INT8 quantization for minimal memory footprint. The 128-bit
/// vector extensions accelerate tensor operations within esp-dl.
pub fn load_model() -> Model {
    // TODO: Load model weights from flash partition
    // TODO: Initialize esp-dl runtime with model graph
    // TODO: Allocate inference scratch buffer in PSRAM
    // TODO: Verify model input matches MODEL_INPUT_SIZE

    defmt::info!(
        "Bird detection model loaded: input {}x{}, INT8 quantized",
        MODEL_INPUT_SIZE.0, MODEL_INPUT_SIZE.1
    );

    todo!("model loading")
}

impl Model {
    /// Run inference on a PPA-downscaled frame.
    ///
    /// Returns zero or more detections with species classification,
    /// confidence scores, and bounding box coordinates (scaled back
    /// to full-resolution display coordinates).
    pub async fn detect(&self, frame: &crate::FrameBuffer) -> heapless::Vec<crate::Detection, 8> {
        // TODO: Copy/map frame data to model input tensor
        // TODO: Call esp_dl_run_inference() via FFI
        // TODO: Post-process: NMS, confidence filtering
        // TODO: Scale bounding boxes from model coords → display coords
        // TODO: Map class indices → species names via label table
        todo!("inference")
    }
}
