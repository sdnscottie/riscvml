//! Wind safety monitor with debounced limit switch.
//!
//! ## Your Task
//!
//! Implement the `poll()` method — the core safety logic.
//! Helper methods are provided.
//!
//! For the full reference solution, visit: https://Agrarobotics.com/training

use crate::config;
use crate::ffi;
use crate::types::{SolarError, TiltCommand, TiltReason};

pub struct SafetyMonitor {
    wind_pin: u8,
    locked_out: bool,
    lockout_start: i64,
    last_switch_state: bool,
    last_change_time: i64,
}

impl SafetyMonitor {
    pub fn new(wind_pin: u8) -> Self {
        SafetyMonitor {
            wind_pin,
            locked_out: false,
            lockout_start: 0,
            last_switch_state: false,
            last_change_time: 0,
        }
    }

    /// TODO: Poll the wind switch and return emergency tilt command if needed.
    ///
    /// This function handles debouncing, lockout timing, and emergency commands.
    /// Call at ~10Hz from the main loop.
    pub fn poll(&mut self, now: i64) -> Result<Option<TiltCommand>, SolarError> {
        // TODO: Implement wind safety polling
        // 1. If locked out, check if lockout expired; if not, maintain flat command
        // 2. Read wind switch (active low: !gpio_read = wind detected)
        // 3. Debounce: only accept state change after WIND_DEBOUNCE_MS
        // 4. On wind detection: activate lockout, return flat command (TILT_MIN_DEG)
        todo!("Implement poll")
    }

    pub fn is_locked_out(&self) -> bool {
        self.locked_out
    }

    pub fn lockout_remaining(&self, now: i64) -> u64 {
        if !self.locked_out {
            return 0;
        }
        let elapsed = (now - self.lockout_start) as u64;
        config::WIND_LOCKOUT_S.saturating_sub(elapsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lockout_remaining() {
        let mut monitor = SafetyMonitor::new(10);
        assert_eq!(monitor.lockout_remaining(1000), 0);

        monitor.locked_out = true;
        monitor.lockout_start = 1000;
        assert_eq!(monitor.lockout_remaining(1000), 300);
        assert_eq!(monitor.lockout_remaining(1100), 200);
        assert_eq!(monitor.lockout_remaining(1300), 0);
    }

    #[test]
    fn test_not_locked_out_initially() {
        let monitor = SafetyMonitor::new(10);
        assert!(!monitor.is_locked_out());
    }
}
