//! MIPI-CSI Camera — Camera Controller Driver (ESP-IDF FFI)
//!
//! Wraps the ESP-IDF Camera Controller Driver to capture raw Bayer frames
//! from a MIPI-CSI camera sensor. The driver handles lane configuration,
//! clock setup, and DMA descriptor chains. Rust owns the frame buffers
//! and passes them downstream with zero-copy semantics.

use esp_idf_sys as _;

/// CSI lane configuration for the connected camera sensor.
pub struct CsiConfig {
    pub lanes: u8,          // 1, 2, or 4 MIPI lanes
    pub clock_freq_hz: u32, // CSI clock frequency
    pub frame_width: u32,
    pub frame_height: u32,
}

impl Default for CsiConfig {
    fn default() -> Self {
        Self {
            lanes: 2,
            clock_freq_hz: 80_000_000,
            frame_width: 1920,
            frame_height: 1080,
        }
    }
}

/// Rust-owned handle to the ESP-IDF camera controller.
pub struct CsiCamera {
    // Raw ESP-IDF camera handle — accessed only through safe wrappers
    _handle: *mut core::ffi::c_void,
    frame_buffers: &'static mut [crate::FrameBuffer],
}

/// Initialize the MIPI-CSI camera with DMA-backed frame buffers.
///
/// The Camera Controller Driver (ESP-IDF C component) manages the hardware,
/// while Rust owns the allocated frame buffers and enforces exclusive access
/// through the borrow checker.
pub fn init(_peripherals: &esp_hal::peripherals::Peripherals) -> CsiCamera {
    let config = CsiConfig::default();

    // TODO: Call esp_camera_controller_init() via FFI
    // TODO: Configure DMA descriptors for frame capture
    // TODO: Allocate frame buffer pool in PSRAM

    defmt::info!(
        "CSI camera initialized: {}x{} @ {} lanes",
        config.frame_width, config.frame_height, config.lanes
    );

    todo!("CSI camera initialization")
}

impl CsiCamera {
    /// Capture a single frame — awaits DMA completion via Embassy.
    ///
    /// Returns exclusive ownership of the frame buffer. The caller must
    /// either pass it to the next pipeline stage or return it to the pool.
    /// This compile-time guarantee prevents use-after-free on DMA buffers.
    pub async fn capture_frame(&mut self) -> crate::FrameBuffer {
        // TODO: Await DMA interrupt via embassy signal
        // TODO: Dequeue completed frame from ESP-IDF driver
        // TODO: Return owned FrameBuffer (zero-copy — same DMA memory)
        todo!("frame capture")
    }
}
