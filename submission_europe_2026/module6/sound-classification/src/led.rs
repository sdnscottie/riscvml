//! RGB LED — Event Category Color Indicator for Sound Classification
//!
//!   Glass break → Red (danger)
//!   Doorbell    → White (neutral)
//!   Dog bark    → Orange (attention)

use crate::detect_db::RgbColor;

pub struct RgbLed {
    current_color: RgbColor,
}

pub fn init(_peripherals: &esp_hal::peripherals::Peripherals) -> RgbLed {
    defmt::info!("RGB LED initialized (sound classification mode)");
    todo!("RGB LED init")
}

impl RgbLed {
    pub fn set_color(&mut self, color: RgbColor) {
        self.current_color = color;
    }
}
