//! Linear actuator control via PWM + H-bridge GPIO.
//!
//! ## Your Task
//!
//! Implement the `move_to_angle()` method and the `proportional_duty()` helper.
//! The H-bridge control helpers are provided.
//!
//! For the full reference solution, visit: https://Agrarobotics.com/training

use crate::config;
use crate::ffi::{self, PwmChannel};
use crate::types::SolarError;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Stopped,
}

pub struct Actuator<'a> {
    pwm: &'a PwmChannel,
    dir_a_pin: u8,
    dir_b_pin: u8,
    enable_pin: u8,
    current_direction: Direction,
}

impl<'a> Actuator<'a> {
    pub fn new(pwm: &'a PwmChannel, dir_a_pin: u8, dir_b_pin: u8, enable_pin: u8) -> Self {
        Actuator {
            pwm,
            dir_a_pin,
            dir_b_pin,
            enable_pin,
            current_direction: Direction::Stopped,
        }
    }

    /// TODO: Move the actuator toward the target angle.
    ///
    /// Use proportional speed control and the TILT_DEADBAND_DEG constant.
    /// Return Ok(true) when within deadband, Ok(false) when still moving.
    pub fn move_to_angle(&mut self, current_deg: f64, target_deg: f64) -> Result<bool, SolarError> {
        // TODO: Implement proportional tilt control
        // 1. Calculate error = target - current
        // 2. If within deadband → stop and return Ok(true)
        // 3. Determine direction (Up or Down)
        // 4. Set speed via proportional_duty()
        // 5. Enable H-bridge
        todo!("Implement move_to_angle")
    }

    pub fn emergency_stop(&mut self) -> Result<(), SolarError> {
        self.stop()
    }

    fn stop(&mut self) -> Result<(), SolarError> {
        ffi::gpio_set(self.enable_pin, false)?;
        ffi::gpio_set(self.dir_a_pin, false)?;
        ffi::gpio_set(self.dir_b_pin, false)?;
        ffi::pwm_set_duty(self.pwm, 0.0)?;
        self.current_direction = Direction::Stopped;
        Ok(())
    }

    fn set_direction(&mut self, dir: Direction) -> Result<(), SolarError> {
        if dir == self.current_direction {
            return Ok(());
        }
        match dir {
            Direction::Up => {
                ffi::gpio_set(self.dir_a_pin, true)?;
                ffi::gpio_set(self.dir_b_pin, false)?;
            }
            Direction::Down => {
                ffi::gpio_set(self.dir_a_pin, false)?;
                ffi::gpio_set(self.dir_b_pin, true)?;
            }
            Direction::Stopped => {
                ffi::gpio_set(self.dir_a_pin, false)?;
                ffi::gpio_set(self.dir_b_pin, false)?;
            }
        }
        self.current_direction = dir;
        Ok(())
    }
}

/// TODO: Calculate PWM duty cycle from angular error.
///
/// Design a stepped control curve with at least 3 speed levels.
fn proportional_duty(error_deg: f64) -> f64 {
    // TODO: Implement proportional speed mapping
    todo!("Implement proportional_duty")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proportional_duty_fast() {
        assert_eq!(proportional_duty(10.0), 100.0);
    }

    #[test]
    fn test_proportional_duty_moderate() {
        assert_eq!(proportional_duty(3.0), 60.0);
    }

    #[test]
    fn test_proportional_duty_slow() {
        assert_eq!(proportional_duty(1.0), 30.0);
    }

    #[test]
    fn test_proportional_duty_deadband() {
        assert_eq!(proportional_duty(0.4), 0.0);
    }
}
