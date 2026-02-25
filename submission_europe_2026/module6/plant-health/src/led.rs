//! RGB LED — Health Status Color Indicator for Plant Assessment
//!
//!   Healthy  → Green
//!   Stressed → Yellow
//!   Diseased → Red

use crate::detect_db::RgbColor;

pub struct RgbLed {
    current_color: RgbColor,
}

pub fn init(_peripherals: &esp_hal::peripherals::Peripherals) -> RgbLed {
    defmt::info!("RGB LED initialized (plant health mode)");
    todo!("RGB LED init")
}

impl RgbLed {
    pub fn set_color(&mut self, color: RgbColor) {
        self.current_color = color;
    }
}
