//! Shared types for the solar controller system.

use std::fmt;

/// A power measurement from one string of solar panels.
#[derive(Debug, Clone, Copy)]
pub struct PowerReading {
    /// String identifier (0, 1, or 2 for strings A, B, C)
    pub string_id: u8,
    /// Measured voltage in volts
    pub voltage_v: f64,
    /// Measured current in amps
    pub current_a: f64,
    /// Calculated power in watts (voltage x current)
    pub power_w: f64,
    /// Unix timestamp of the reading
    pub timestamp: i64,
}

/// Command to move the tilt actuator to a target angle.
#[derive(Debug, Clone, Copy)]
pub struct TiltCommand {
    /// Target tilt angle in degrees
    pub target_deg: f64,
    /// Why the tilt is changing
    pub reason: TiltReason,
}

/// Reason for a tilt adjustment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TiltReason {
    /// Active sun tracking (optimal angle calculation)
    SunTracking,
    /// Nighttime seasonal default
    SeasonalDefault,
    /// Emergency wind safety (go flat)
    WindSafety,
    /// Manual override
    Manual,
}

/// Status of one string of solar panels.
#[derive(Debug, Clone, Copy, Default)]
pub struct StringStatus {
    /// Measured voltage in volts
    pub voltage_v: f64,
    /// Measured current in amps
    pub current_a: f64,
    /// Calculated power in watts
    pub power_w: f64,
}

/// Full system status snapshot.
#[derive(Debug, Clone)]
pub struct SystemStatus {
    /// Current tilt angle in degrees
    pub tilt_deg: f64,
    /// Total system power in watts (sum of all strings)
    pub total_power_w: f64,
    /// Per-string status
    pub strings: [StringStatus; 3],
    /// Whether wind lockout is active
    pub wind_lockout: bool,
    /// Unix timestamp
    pub timestamp: i64,
}

/// Unified error type for all solar controller operations.
#[derive(Debug)]
pub enum SolarError {
    /// I2C communication failure
    I2c(&'static str),
    /// ADC read failure
    Adc(&'static str),
    /// PWM configuration failure
    Pwm(&'static str),
    /// GPIO operation failure
    Gpio(&'static str),
    /// Database error
    Db(String),
    /// Operation timed out
    Timeout,
}

impl fmt::Display for SolarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolarError::I2c(msg) => write!(f, "I2C error: {msg}"),
            SolarError::Adc(msg) => write!(f, "ADC error: {msg}"),
            SolarError::Pwm(msg) => write!(f, "PWM error: {msg}"),
            SolarError::Gpio(msg) => write!(f, "GPIO error: {msg}"),
            SolarError::Db(msg) => write!(f, "Database error: {msg}"),
            SolarError::Timeout => write!(f, "Operation timed out"),
        }
    }
}

impl std::error::Error for SolarError {}

impl From<rusqlite::Error> for SolarError {
    fn from(e: rusqlite::Error) -> Self {
        SolarError::Db(e.to_string())
    }
}
