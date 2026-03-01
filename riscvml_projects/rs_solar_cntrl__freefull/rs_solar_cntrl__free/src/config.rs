//! System configuration constants for the rs_solar_cntrl solar controller.
//!
//! All physical, electrical, and hardware parameters in one place.

// === Site Location ===

/// Latitude in degrees north (Bad Schwalbach, Germany)
pub const SITE_LATITUDE_DEG: f64 = 50.1;
/// Longitude in degrees east (Bad Schwalbach, Germany)
pub const SITE_LONGITUDE_DEG: f64 = 8.1;

// === Tilt Range ===

/// Minimum tilt angle in degrees (greenhouse/summer position)
pub const TILT_MIN_DEG: f64 = 8.0;
/// Maximum tilt angle in degrees (house roof/winter position)
pub const TILT_MAX_DEG: f64 = 24.0;
/// Default midpoint tilt angle
pub const TILT_DEFAULT_DEG: f64 = 16.0;
/// Deadband: don't move actuator if within this many degrees of target
pub const TILT_DEADBAND_DEG: f64 = 0.5;

// === Panel Electrical Specs (TrinaSolar TSM-620NEG19RC.20) ===

/// Number of panels total
pub const PANEL_COUNT: usize = 12;
/// Number of strings
pub const STRING_COUNT: usize = 3;
/// Panels per string (in series)
pub const PANELS_PER_STRING: usize = 4;
/// Single panel Vmp (volts)
pub const PANEL_VMP: f64 = 41.4;
/// Single panel Imp (amps)
pub const PANEL_IMP: f64 = 14.99;
/// Single panel Wp
pub const PANEL_WP: f64 = 620.0;
/// String Vmp = 4 x 41.4 V
pub const STRING_VMP: f64 = 165.6;
/// String Imp (same as panel Imp for series connection)
pub const STRING_IMP: f64 = 14.99;
/// Total system Wp
pub const SYSTEM_WP: f64 = 7440.0;

// === ADC Configuration ===

/// Voltage divider ratio (100:1 for high-voltage string measurement)
pub const VOLTAGE_DIVIDER_RATIO: f64 = 100.0;
/// ACS712 current sensor sensitivity (mV per amp)
pub const CURRENT_SENSOR_MV_PER_AMP: f64 = 100.0;
/// ACS712 zero-current output voltage (mV) — midpoint of 3.3V supply
pub const CURRENT_SENSOR_ZERO_MV: f64 = 1650.0;
/// ADC resolution (12-bit)
pub const ADC_RESOLUTION_BITS: u32 = 12;
/// ADC reference voltage (mV)
pub const ADC_VREF_MV: f64 = 3300.0;

// === I2C Configuration ===

/// ADXL345 inclinometer I2C address
pub const INCLINOMETER_I2C_ADDR: u8 = 0x53;
/// I2C clock speed (Hz)
pub const I2C_CLOCK_HZ: u32 = 400_000;

// === GPIO Pin Assignments (ESP32-P4) ===

/// I2C SDA pin
pub const PIN_I2C_SDA: u8 = 4;
/// I2C SCL pin
pub const PIN_I2C_SCL: u8 = 5;
/// PWM output for actuator speed
pub const PIN_ACTUATOR_PWM: u8 = 6;
/// H-bridge direction pin A
pub const PIN_HBRIDGE_DIR_A: u8 = 7;
/// H-bridge direction pin B
pub const PIN_HBRIDGE_DIR_B: u8 = 8;
/// H-bridge enable pin
pub const PIN_HBRIDGE_EN: u8 = 9;
/// Wind limit switch input (active low)
pub const PIN_WIND_SWITCH: u8 = 10;
/// Status LED pin
pub const PIN_STATUS_LED: u8 = 11;
/// ADC pins for string voltage measurements (String A, B, C)
pub const PIN_ADC_VOLTAGE: [u8; 3] = [12, 13, 14];
/// ADC pins for string current measurements (String A, B, C)
pub const PIN_ADC_CURRENT: [u8; 3] = [15, 16, 17];

// === Timing ===

/// ADC sampling interval (seconds)
pub const ADC_SAMPLE_INTERVAL_S: u64 = 10;
/// Tilt recalculation interval (seconds) — every 15 minutes
pub const TILT_RECALC_INTERVAL_S: u64 = 900;
/// Status snapshot interval (seconds) — every 5 minutes
pub const STATUS_SNAPSHOT_INTERVAL_S: u64 = 300;
/// Wind switch debounce time (milliseconds)
pub const WIND_DEBOUNCE_MS: u64 = 500;
/// Wind lockout duration (seconds) — 5 minutes after wind event
pub const WIND_LOCKOUT_S: u64 = 300;

// === Database ===

/// SQLite database file path (SD card on ESP32-P4)
pub const DB_PATH: &str = "/sdcard/solar_telemetry.db";

// === Sun Position ===

/// Sun elevation threshold for "daytime" (civil twilight)
pub const DAYLIGHT_ELEVATION_DEG: f64 = -6.0;
