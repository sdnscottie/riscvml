//! MIPI-DSI Display — DMA-driven frame output
//!
//! Drives a MIPI-DSI display panel for live camera preview at 30-60 FPS.
//! Frames arrive from the ISP pipeline via zero-copy buffer handoff.
//! The display driver manages double-buffering: one buffer is being
//! transmitted via DMA while the next frame is being prepared.

use esp_idf_sys as _;

pub struct DisplayConfig {
    pub width: u16,
    pub height: u16,
    pub refresh_hz: u8,
    pub dsi_lanes: u8,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 600,
            refresh_hz: 60,
            dsi_lanes: 2,
        }
    }
}

/// Rust-owned handle to the MIPI-DSI display.
pub struct DsiDisplay {
    _handle: *mut core::ffi::c_void,
    config: DisplayConfig,
}

/// Initialize the MIPI-DSI display with double-buffered DMA output.
pub fn init(_peripherals: &esp_hal::peripherals::Peripherals) -> DsiDisplay {
    let config = DisplayConfig::default();

    // TODO: Initialize MIPI-DSI host via esp_lcd_new_panel_dsi()
    // TODO: Configure display timing parameters
    // TODO: Allocate double-buffer in PSRAM for DMA

    defmt::info!(
        "DSI display initialized: {}x{} @ {}Hz",
        config.width, config.height, config.refresh_hz
    );

    todo!("DSI display initialization")
}

impl DsiDisplay {
    /// Push a processed frame to the display via DMA.
    ///
    /// Takes ownership of the frame buffer. The previous frame's DMA
    /// transfer must complete before this buffer can be submitted.
    /// Embassy async/await handles the wait without blocking.
    pub async fn push_frame(&mut self, frame: crate::FrameBuffer) {
        // TODO: Wait for previous DMA transfer completion
        // TODO: Submit new frame buffer to DSI DMA
        // TODO: Return previous frame buffer to pool
        todo!("display frame push")
    }
}
