//! RGB LED — Threat-Level Color Indicator for Object Detection
//!
//! Same LED hardware as bird detection, different color semantics:
//!   Person  → Yellow (caution)
//!   Vehicle → Cyan (moving obstacle)
//!   Animal  → Magenta (wildlife)

use crate::detect_db::RgbColor;

pub struct RgbLed {
    current_color: RgbColor,
}

pub fn init(_peripherals: &esp_hal::peripherals::Peripherals) -> RgbLed {
    defmt::info!("RGB LED initialized (object detection mode)");
    todo!("RGB LED init")
}

impl RgbLed {
    pub fn set_color(&mut self, color: RgbColor) {
        self.current_color = color;
        // TODO: Set PWM duty cycles
    }
}
