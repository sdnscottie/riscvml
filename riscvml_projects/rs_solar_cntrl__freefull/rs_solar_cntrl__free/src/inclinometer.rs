//! ADXL345 inclinometer driver over I2C.
//!
//! ## Your Task
//!
//! Implement the following functions:
//! - `read_angle()` — read tilt angle with multi-sample averaging
//! - `raw_to_angle()` — convert XYZ acceleration to tilt angle
//!
//! The I2C initialization (`new()`) is provided.
//! Use the unit tests as your specification.
//!
//! For the full reference solution, visit: https://Agrarobotics.com/training

use crate::config;
use crate::ffi::{self, I2cBus};
use crate::types::SolarError;

// ADXL345 register addresses
const REG_POWER_CTL: u8 = 0x2D;
const REG_DATA_FORMAT: u8 = 0x31;
const REG_DATAX0: u8 = 0x32;
const REG_BW_RATE: u8 = 0x2C;

/// Number of samples to average for noise rejection.
const AVERAGE_SAMPLES: usize = 8;

/// ADXL345 inclinometer driver.
pub struct Inclinometer<'a> {
    bus: &'a I2cBus,
    addr: u8,
    /// Calibration offset in degrees (added to raw reading)
    offset_deg: f64,
}

impl<'a> Inclinometer<'a> {
    /// Create and initialize the ADXL345 (provided complete).
    pub fn new(bus: &'a I2cBus) -> Result<Self, SolarError> {
        let addr = config::INCLINOMETER_I2C_ADDR;
        ffi::i2c_write(bus, addr, &[REG_POWER_CTL, 0x08])?;
        ffi::i2c_write(bus, addr, &[REG_DATA_FORMAT, 0x08])?;
        ffi::i2c_write(bus, addr, &[REG_BW_RATE, 0x0A])?;

        Ok(Inclinometer {
            bus,
            addr,
            offset_deg: 0.0,
        })
    }

    pub fn set_calibration_offset(&mut self, offset_deg: f64) {
        self.offset_deg = offset_deg;
    }

    /// TODO: Read the current tilt angle, averaged over AVERAGE_SAMPLES readings.
    ///
    /// HINT: Call `read_raw_xyz()` in a loop AVERAGE_SAMPLES times,
    /// convert each reading with `raw_to_angle()`, sum them, divide by
    /// AVERAGE_SAMPLES, then add `self.offset_deg`.
    pub fn read_angle(&self) -> Result<f64, SolarError> {
        // TODO: Implement multi-sample averaged angle reading
        // Step 1: Loop AVERAGE_SAMPLES times, calling read_raw_xyz() each time
        // Step 2: Convert each (x, y, z) to angle using raw_to_angle()
        // Step 3: Sum all angles, divide by AVERAGE_SAMPLES
        // Step 4: Add self.offset_deg to the averaged result
        todo!("Implement read_angle — see hints above")
    }

    /// Read raw XYZ acceleration values from the ADXL345 (provided complete).
    fn read_raw_xyz(&self) -> Result<(i16, i16, i16), SolarError> {
        let mut buf = [0u8; 6];
        ffi::i2c_read(self.bus, self.addr, REG_DATAX0, &mut buf)?;

        let x = i16::from_le_bytes([buf[0], buf[1]]);
        let y = i16::from_le_bytes([buf[2], buf[3]]);
        let z = i16::from_le_bytes([buf[4], buf[5]]);

        Ok((x, y, z))
    }
}

/// TODO: Convert raw XYZ acceleration to tilt angle in degrees.
///
/// HINT: Use atan2(x, z) to compute the angle. The Y axis is lateral
/// (east-west) and not used for north-south tilt measurement.
/// Convert from radians to degrees: angle_deg = angle_rad * 180 / PI
fn raw_to_angle(x: i16, y: i16, z: i16) -> f64 {
    let _ = y; // Y axis is lateral, not used for tilt
    // TODO: Implement angle conversion
    // Step 1: Convert x and z to f64
    // Step 2: Use libm::atan2(x_f, z_f) to get angle in radians
    // Step 3: Convert radians to degrees
    todo!("Implement raw_to_angle — use atan2(x, z)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_to_angle_level() {
        let angle = raw_to_angle(0, 0, 256);
        assert!(
            angle.abs() < 1.0,
            "Level sensor should read ~0 deg, got {angle}"
        );
    }

    #[test]
    fn test_raw_to_angle_tilted() {
        let angle = raw_to_angle(66, 0, 247);
        assert!(
            (angle - 15.0).abs() < 1.0,
            "Should read ~15 deg, got {angle}"
        );
    }

    #[test]
    fn test_raw_to_angle_steep() {
        let angle = raw_to_angle(181, 0, 181);
        assert!(
            (angle - 45.0).abs() < 1.0,
            "Should read ~45 deg, got {angle}"
        );
    }
}
