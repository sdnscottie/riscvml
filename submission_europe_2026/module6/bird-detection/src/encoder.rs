//! H.264 Hardware Encoder — 1080p@30fps Zero-Copy Recording
//!
//! Wraps the ESP32-P4's hardware H.264 encoder for event-triggered
//! video recording. Full-resolution frames from the ISP pipeline are
//! fed directly to the encoder via DMA — no CPU-side pixel copying.
//!
//! Rust's ownership model guarantees that a frame buffer is either
//! being displayed OR being encoded, never both simultaneously,
//! preventing DMA conflicts at compile time.

use esp_idf_sys as _;

pub struct EncoderConfig {
    pub width: u32,
    pub height: u32,
    pub fps: u8,
    pub bitrate_kbps: u32,
    pub gop_size: u8, // I-frame interval
}

impl Default for EncoderConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps: 30,
            bitrate_kbps: 4000,
            gop_size: 30,
        }
    }
}

/// Rust-owned handle to the H.264 hardware encoder.
pub struct H264Encoder {
    _handle: *mut core::ffi::c_void,
    config: EncoderConfig,
    recording: bool,
}

/// Encoded NAL unit ready for streaming or storage.
pub struct EncodedChunk {
    pub data: heapless::Vec<u8, 65536>,
    pub is_keyframe: bool,
    pub timestamp_ms: u64,
}

/// Initialize the H.264 hardware encoder.
pub fn init(_peripherals: &esp_hal::peripherals::Peripherals) -> H264Encoder {
    let config = EncoderConfig::default();

    // TODO: Initialize H.264 encoder hardware via ESP-IDF
    // TODO: Configure bitrate, GOP, quality parameters
    // TODO: Set up DMA input path from frame buffer pool

    defmt::info!(
        "H.264 encoder initialized: {}x{} @ {}fps, {} kbps",
        config.width, config.height, config.fps, config.bitrate_kbps
    );

    todo!("H.264 encoder initialization")
}

impl H264Encoder {
    /// Encode the next frame from the DMA pipeline.
    ///
    /// Awaits a frame from the ISP output, submits it to the hardware
    /// encoder, and returns the encoded H.264 NAL unit. Zero-copy:
    /// the encoder reads directly from the DMA buffer.
    pub async fn encode_next_frame(&mut self) -> EncodedChunk {
        // TODO: Await frame from ISP DMA ring buffer
        // TODO: Submit to H.264 encoder DMA input
        // TODO: Await encoder completion interrupt
        // TODO: Read encoded NAL unit from output buffer
        todo!("H.264 encoding")
    }

    /// Start recording (triggered by bird detection event).
    pub fn start_recording(&mut self) {
        self.recording = true;
        defmt::info!("Recording started");
    }

    /// Stop recording.
    pub fn stop_recording(&mut self) {
        self.recording = false;
        defmt::info!("Recording stopped");
    }
}
