//! Tilt angle scheduler — recalculates optimal panel tilt periodically.
//!
//! ## Your Task
//!
//! Implement the `tick()` and `calculate_target()` methods.
//!
//! For the full reference solution, visit: https://Agrarobotics.com/training

use crate::config;
use crate::sun_position::{self, SunPosition};
use crate::types::{TiltCommand, TiltReason};

pub struct Scheduler {
    last_target_deg: f64,
    last_recalc_time: i64,
}

impl Scheduler {
    pub fn new(initial_tilt_deg: f64) -> Self {
        Scheduler {
            last_target_deg: initial_tilt_deg,
            last_recalc_time: 0,
        }
    }

    /// TODO: Tick the scheduler — check if a tilt recalculation is due.
    ///
    /// Return Some(TiltCommand) if tilt should change, None otherwise.
    pub fn tick(&mut self, now: i64) -> Option<TiltCommand> {
        // TODO: Implement scheduler tick
        // 1. Check if enough time has elapsed since last recalc (TILT_RECALC_INTERVAL_S)
        // 2. Calculate sun position using sun_position::sun_position()
        // 3. Determine target via calculate_target()
        // 4. Only issue command if delta > TILT_DEADBAND_DEG
        todo!("Implement tick")
    }

    /// TODO: Calculate target tilt based on sun position.
    ///
    /// Return (target_deg, reason).
    fn calculate_target(&self, sun: &SunPosition, now: i64) -> (f64, TiltReason) {
        // TODO: Implement target calculation
        // If optimal_tilt returns Some → use it (SunTracking)
        // If None (night) → use seasonal_default (SeasonalDefault)
        todo!("Implement calculate_target")
    }

    pub fn force_recalc(&mut self) {
        self.last_recalc_time = 0;
    }

    pub fn last_target(&self) -> f64 {
        self.last_target_deg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_initial_tick() {
        let mut sched = Scheduler::new(16.0);
        let cmd = sched.tick(1718971200);
        assert!(cmd.is_some(), "First tick should produce a command");
        let cmd = cmd.unwrap();
        assert_eq!(cmd.reason, TiltReason::SunTracking);
    }

    #[test]
    fn test_scheduler_respects_interval() {
        let mut sched = Scheduler::new(16.0);
        let _ = sched.tick(1718971200);
        let cmd = sched.tick(1718971200 + 60);
        assert!(cmd.is_none(), "Should not recalculate within interval");
    }

    #[test]
    fn test_scheduler_nighttime_seasonal() {
        let mut sched = Scheduler::new(8.0);
        let cmd = sched.tick(1734742800);
        assert!(cmd.is_some(), "Night tick should produce seasonal command");
        let cmd = cmd.unwrap();
        assert_eq!(cmd.reason, TiltReason::SeasonalDefault);
        assert!(
            cmd.target_deg > 20.0,
            "Winter night should target high tilt"
        );
    }
}
