//! ADC monitoring for solar string voltage and current.
//!
//! ## Your Task
//!
//! Implement the following functions:
//! - `read_all_strings()` — read voltage and current for all 3 strings
//! - `adc_mv_to_string_voltage()` — convert ADC millivolts to actual volts
//! - `adc_mv_to_string_current()` — convert ADC millivolts to actual amps
//!
//! Use the unit tests as your specification.
//!
//! For the full reference solution, visit: https://Agrarobotics.com/training

use heapless::Vec;

use crate::config;
use crate::ffi;
use crate::types::{PowerReading, SolarError, StringStatus};

/// ADC monitor for the 3-string solar array.
pub struct AdcMonitor {
    voltage_channels: [u8; 3],
    current_channels: [u8; 3],
}

impl AdcMonitor {
    pub fn new(voltage_channels: [u8; 3], current_channels: [u8; 3]) -> Self {
        AdcMonitor {
            voltage_channels,
            current_channels,
        }
    }

    /// TODO: Read all three strings and return power readings.
    ///
    /// HINT: For each string (0..STRING_COUNT):
    /// 1. Call ffi::adc_read_calibrated() for the voltage channel
    /// 2. Call ffi::adc_read_calibrated() for the current channel
    /// 3. Convert both using adc_mv_to_string_voltage/current
    /// 4. Calculate power = voltage * current
    /// 5. Push a PowerReading into the Vec
    pub fn read_all_strings(&self, timestamp: i64) -> Result<Vec<PowerReading, 3>, SolarError> {
        // TODO: Implement this function
        todo!("Implement read_all_strings — read 3 strings from ADC")
    }

    pub fn read_system_status(
        &self,
        timestamp: i64,
    ) -> Result<([StringStatus; 3], f64), SolarError> {
        let readings = self.read_all_strings(timestamp)?;
        let mut strings = [StringStatus::default(); 3];
        let mut total_power = 0.0;

        for reading in &readings {
            let idx = reading.string_id as usize;
            strings[idx] = StringStatus {
                voltage_v: reading.voltage_v,
                current_a: reading.current_a,
                power_w: reading.power_w,
            };
            total_power += reading.power_w;
        }

        Ok((strings, total_power))
    }
}

/// TODO: Convert ADC millivolts to actual string voltage.
///
/// HINT: The string voltage (up to ~200V) is divided by a 100:1 resistor
/// divider before reaching the ADC. So:
///   actual_voltage = adc_mv * VOLTAGE_DIVIDER_RATIO / 1000
///
/// Example: 1656 mV at ADC → 165.6V actual string voltage
pub fn adc_mv_to_string_voltage(adc_mv: u32) -> f64 {
    // TODO: Implement voltage conversion
    todo!("Implement adc_mv_to_string_voltage")
}

/// TODO: Convert ADC millivolts to actual string current.
///
/// HINT: The ACS712 current sensor outputs voltage centered at Vcc/2:
///   - Zero current = 1650 mV (CURRENT_SENSOR_ZERO_MV)
///   - Sensitivity = 100 mV per amp (CURRENT_SENSOR_MV_PER_AMP)
///   - current = (adc_mv - 1650) / 100
///
/// Example: 3149 mV → (3149 - 1650) / 100 = 14.99 A
pub fn adc_mv_to_string_current(adc_mv: u32) -> f64 {
    // TODO: Implement current conversion
    todo!("Implement adc_mv_to_string_current")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voltage_conversion_nominal() {
        let voltage = adc_mv_to_string_voltage(1656);
        assert!(
            (voltage - 165.6).abs() < 0.1,
            "Expected ~165.6V, got {voltage}"
        );
    }

    #[test]
    fn test_voltage_conversion_zero() {
        assert_eq!(adc_mv_to_string_voltage(0), 0.0);
    }

    #[test]
    fn test_current_conversion_nominal() {
        let current = adc_mv_to_string_current(3149);
        assert!(
            (current - 14.99).abs() < 0.1,
            "Expected ~14.99A, got {current}"
        );
    }

    #[test]
    fn test_current_conversion_zero() {
        let current = adc_mv_to_string_current(1650);
        assert!(
            current.abs() < 0.01,
            "Zero current should be ~0A, got {current}"
        );
    }

    #[test]
    fn test_current_conversion_negative() {
        let current = adc_mv_to_string_current(1550);
        assert!(current < 0.0, "Should be negative for reverse current");
    }
}
