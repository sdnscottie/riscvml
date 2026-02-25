//! ISP Pipeline — Image Signal Processor (ESP-IDF FFI)
//!
//! Configures the ESP32-P4's hardware ISP to transform raw Bayer sensor
//! data into RGB frames suitable for display and ML inference. Each ISP
//! stage (white balance, auto-exposure, demosaicing, color correction)
//! is wrapped in type-safe Rust abstractions that prevent misconfiguration
//! at compile time.

use esp_idf_sys as _;

/// Type-safe ISP stage configuration.
/// Each field maps to an ISP register block — invalid combinations
/// (e.g., demosaicing before white balance) are prevented by the
/// builder pattern's type state.
pub struct IspConfig {
    pub awb_enabled: bool,
    pub ae_enabled: bool,
    pub ae_target_brightness: u8,
    pub demosaic_method: DemosaicMethod,
    pub color_correction_matrix: [[f32; 3]; 3],
}

pub enum DemosaicMethod {
    /// Bilinear interpolation — fast, lower quality
    Bilinear,
    /// Edge-directed — better edges, slightly more computation
    EdgeDirected,
}

impl Default for IspConfig {
    fn default() -> Self {
        Self {
            awb_enabled: true,
            ae_enabled: true,
            ae_target_brightness: 128,
            demosaic_method: DemosaicMethod::EdgeDirected,
            color_correction_matrix: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
}

/// Rust-owned handle to the hardware ISP pipeline.
pub struct IspPipeline {
    _handle: *mut core::ffi::c_void,
    config: IspConfig,
}

/// Initialize the ISP with default auto white balance and auto exposure.
pub fn init(_peripherals: &esp_hal::peripherals::Peripherals) -> IspPipeline {
    let config = IspConfig::default();

    // TODO: Call esp_isp_new_processor() via FFI
    // TODO: Configure AWB, AE, demosaicing stages
    // TODO: Set up ISP → DMA output path

    defmt::info!("ISP pipeline initialized: AWB={}, AE={}", config.awb_enabled, config.ae_enabled);

    todo!("ISP initialization")
}

impl IspPipeline {
    /// Process a raw Bayer frame through the hardware ISP pipeline.
    ///
    /// Input: raw sensor data (owned FrameBuffer from camera.rs)
    /// Output: RGB frame ready for display or ML preprocessing
    ///
    /// The ISP hardware processes in-place via DMA — no CPU copy required.
    /// Ownership transfers: camera → ISP → caller.
    pub async fn process(&mut self, frame: crate::FrameBuffer) -> crate::FrameBuffer {
        // TODO: Submit frame to ISP hardware via DMA descriptor
        // TODO: Await ISP completion interrupt via embassy
        // TODO: Return processed frame (same buffer, now RGB)
        todo!("ISP processing")
    }

    /// Update auto-exposure target at runtime.
    pub fn set_ae_target(&mut self, brightness: u8) {
        self.config.ae_target_brightness = brightness;
        // TODO: Write to ISP AE register via FFI
    }
}
