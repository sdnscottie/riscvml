//! PPA — Pixel Processing Accelerator (ESP-IDF FFI)
//!
//! The ESP32-P4's PPA provides hardware-accelerated 2D operations:
//! scaling, rotation, mirroring, and alpha blending. This module wraps
//! PPA for two pipeline roles:
//!
//!   1. Downscale: Resize full-resolution camera frames to ML model
//!      input dimensions (e.g., 1920x1080 → 320x320) without CPU load.
//!
//!   2. Alpha blend: Overlay bounding boxes and confidence labels onto
//!      the display preview frame.

use esp_idf_sys as _;
use embassy_sync::channel::Channel;

/// Queue for bounding box overlay requests from inference → display.
pub static OVERLAY_QUEUE: Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    crate::BoundingBox,
    4,
> = Channel::new();

pub struct PpaContext {
    _handle: *mut core::ffi::c_void,
}

/// Initialize the PPA hardware accelerator.
pub fn init(_peripherals: &esp_hal::peripherals::Peripherals) -> PpaContext {
    // TODO: Call ppa_client_register() via FFI
    // TODO: Configure DMA for PPA input/output

    defmt::info!("PPA initialized");
    todo!("PPA initialization")
}

impl PpaContext {
    /// Hardware-accelerated downscale for ML inference input.
    ///
    /// Resizes the full-resolution frame to the target dimensions using
    /// PPA's bilinear scaling engine. Returns a new buffer at model
    /// input resolution — the original frame is not consumed (borrowed).
    pub async fn downscale(
        &mut self,
        frame: &crate::FrameBuffer,
        target_size: (u32, u32),
    ) -> crate::FrameBuffer {
        // TODO: Configure PPA scale-rotate-process (SRP) client
        // TODO: Set input/output DMA descriptors
        // TODO: Await PPA completion interrupt via embassy
        todo!("PPA downscale")
    }
}

/// Overlay bounding boxes onto the display frame via PPA alpha blending.
///
/// This is the "Visualize" step of the Detect → Visualize → React pattern.
/// PPA blends a semi-transparent rectangle and label onto the live preview
/// without CPU-bound pixel manipulation.
pub async fn blend_overlay(
    ppa: &mut PpaContext,
    display_frame: &mut crate::FrameBuffer,
    bbox: &crate::BoundingBox,
    label: &str,
    confidence: f32,
) {
    // TODO: Render bbox rectangle into overlay buffer
    // TODO: Render text label + confidence via pre-rasterized font atlas
    // TODO: PPA alpha blend overlay onto display frame
    // TODO: Use PPA_BLEND_MODE_ARGB for transparency

    defmt::debug!(
        "Overlay: {} ({:.0}%) at [{},{} {}x{}]",
        label, confidence * 100.0,
        bbox.x, bbox.y, bbox.w, bbox.h,
    );

    todo!("PPA alpha blend overlay")
}
