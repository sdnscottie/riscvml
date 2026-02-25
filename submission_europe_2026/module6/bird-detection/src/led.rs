//! RGB LED Indicator — Visual Species Identification
//!
//! Drives an RGB LED to display a species-specific color when a bird is
//! detected. The color mapping is stored in the `bird_led_colors` table
//! of riscvml_detect.db and looked up via detect_db::get_led_color().
//!
//! This implements the "React" step of the Detect → Visualize → React
//! pattern — providing instant visual feedback of classification results.

use crate::detect_db::RgbColor;

/// RGB LED driver using PWM channels for smooth color output.
pub struct RgbLed {
    // PWM channel handles for red, green, blue
    _r_channel: u8,
    _g_channel: u8,
    _b_channel: u8,
    current_color: RgbColor,
}

/// Initialize the RGB LED on three PWM-capable GPIO pins.
pub fn init(_peripherals: &esp_hal::peripherals::Peripherals) -> RgbLed {
    // TODO: Configure three LEDC PWM channels (one per color)
    // TODO: Set PWM frequency (e.g., 5 kHz for flicker-free output)
    // TODO: Default to off (0, 0, 0)

    defmt::info!("RGB LED initialized");

    todo!("RGB LED initialization")
}

impl RgbLed {
    /// Set the LED to a specific RGB color.
    ///
    /// Smoothly transitions from the current color to the target color
    /// using PWM duty cycle adjustment. Called by the inference task
    /// each time a bird is detected.
    pub fn set_color(&mut self, color: RgbColor) {
        self.current_color = color;

        // TODO: Set LEDC duty for R channel (color.r / 255 * max_duty)
        // TODO: Set LEDC duty for G channel
        // TODO: Set LEDC duty for B channel

        defmt::debug!(
            "LED → ({}, {}, {})",
            color.r, color.g, color.b
        );
    }

    /// Turn off the LED (no detection active).
    pub fn off(&mut self) {
        self.set_color(RgbColor { r: 0, g: 0, b: 0 });
    }
}
