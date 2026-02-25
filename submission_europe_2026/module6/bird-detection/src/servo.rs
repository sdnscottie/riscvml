//! PCA9685 Pan/Tilt Servos — Camera Tracking
//!
//! Drives pan and tilt servos via a PCA9685 I2C PWM driver to track
//! detected birds. Detection bounding box coordinates are converted to
//! servo angles that center the camera on the target.
//!
//! Reuses the PCA9685 abstractions from Module 2 (I2C peripheral control),
//! demonstrating curriculum continuity from basic servo exercises to the
//! full capstone pipeline.

/// PCA9685 I2C address (default: 0x40).
const PCA9685_ADDR: u8 = 0x40;

/// Servo pulse width range in microseconds.
const SERVO_MIN_US: u16 = 500;
const SERVO_MAX_US: u16 = 2500;

/// Pan/tilt servo controller.
pub struct PanTiltServos {
    // I2C bus handle for PCA9685 communication
    _i2c: esp_hal::i2c::I2c<'static>,
    pan_channel: u8,
    tilt_channel: u8,
    current_pan: f32,   // degrees: -90..+90
    current_tilt: f32,  // degrees: -45..+45
}

/// Initialize the PCA9685 and configure pan/tilt servo channels.
pub fn init(_peripherals: &esp_hal::peripherals::Peripherals) -> PanTiltServos {
    // TODO: Initialize I2C bus at 400 kHz
    // TODO: Reset PCA9685 and set PWM frequency to 50 Hz
    // TODO: Configure pan channel (0) and tilt channel (1)
    // TODO: Center both servos (0°, 0°)

    defmt::info!("PCA9685 pan/tilt servos initialized");

    todo!("servo initialization")
}

impl PanTiltServos {
    /// Track a detected bird by centering the camera on the bounding box.
    ///
    /// Converts the bounding box center (in display pixel coordinates)
    /// to pan/tilt servo angles using a proportional controller.
    /// Smoothing prevents jitter from frame-to-frame detection noise.
    pub async fn track(&mut self, bbox: crate::BoundingBox) {
        // Calculate bounding box center offset from frame center
        let frame_cx = 960.0_f32; // half of 1920
        let frame_cy = 540.0_f32; // half of 1080

        let det_cx = bbox.x as f32 + bbox.w as f32 / 2.0;
        let det_cy = bbox.y as f32 + bbox.h as f32 / 2.0;

        let err_x = (det_cx - frame_cx) / frame_cx; // -1.0..+1.0
        let err_y = (det_cy - frame_cy) / frame_cy;

        // Proportional servo adjustment (smoothed)
        let pan_step = err_x * 5.0;  // degrees per error unit
        let tilt_step = err_y * 3.0;

        self.current_pan = (self.current_pan + pan_step).clamp(-90.0, 90.0);
        self.current_tilt = (self.current_tilt + tilt_step).clamp(-45.0, 45.0);

        self.set_pan(self.current_pan);
        self.set_tilt(self.current_tilt);
    }

    fn set_pan(&mut self, degrees: f32) {
        let _pulse = Self::degrees_to_pulse(degrees, -90.0, 90.0);
        // TODO: Write pulse width to PCA9685 pan channel via I2C
    }

    fn set_tilt(&mut self, degrees: f32) {
        let _pulse = Self::degrees_to_pulse(degrees, -45.0, 45.0);
        // TODO: Write pulse width to PCA9685 tilt channel via I2C
    }

    fn degrees_to_pulse(degrees: f32, min_deg: f32, max_deg: f32) -> u16 {
        let ratio = (degrees - min_deg) / (max_deg - min_deg);
        let pulse = SERVO_MIN_US as f32 + ratio * (SERVO_MAX_US - SERVO_MIN_US) as f32;
        pulse as u16
    }
}
