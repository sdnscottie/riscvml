//! Solar position calculator.
//!
//! Pure-math solar position algorithm using `libm` trig functions.
//! Computes sun elevation and azimuth from UTC timestamp and geographic
//! coordinates, then derives optimal panel tilt angle.
//!
//! ## Your Task
//!
//! Implement the following functions:
//! - `optimal_tilt()` — calculate the best panel angle from sun elevation
//! - `seasonal_default()` — interpolate tilt angle from day of year
//!
//! The sun position calculation itself (`sun_position()`) is provided.
//! Use the unit tests as your specification — when all tests pass, you're done.
//!
//! For the full reference solution, visit: https://Agrarobotics.com/training

use libm::{acos, asin, cos, floor, sin};

use crate::config;

/// Degrees to radians conversion factor.
const DEG_TO_RAD: f64 = core::f64::consts::PI / 180.0;
/// Radians to degrees conversion factor.
const RAD_TO_DEG: f64 = 180.0 / core::f64::consts::PI;

/// Solar position result.
#[derive(Debug, Clone, Copy)]
pub struct SunPosition {
    /// Sun elevation angle above horizon (degrees, negative = below horizon)
    pub elevation_deg: f64,
    /// Sun azimuth from north, clockwise (degrees: 0=N, 90=E, 180=S, 270=W)
    pub azimuth_deg: f64,
}

/// Calculate the sun's position for a given UTC timestamp and location.
///
/// This function is provided complete — you don't need to modify it.
pub fn sun_position(unix_secs: i64, lat_deg: f64, lon_deg: f64) -> SunPosition {
    let jd = 2440587.5 + (unix_secs as f64 / 86400.0);
    let n = jd - 2451545.0;

    let l = normalize_degrees(280.460 + 0.9856474 * n);
    let g = normalize_degrees(357.528 + 0.9856003 * n);
    let g_rad = g * DEG_TO_RAD;

    let lambda = l + 1.915 * sin(g_rad) + 0.020 * sin(2.0 * g_rad);
    let lambda_rad = lambda * DEG_TO_RAD;

    let epsilon = 23.439 - 0.0000004 * n;
    let epsilon_rad = epsilon * DEG_TO_RAD;

    let decl_rad = asin(sin(epsilon_rad) * sin(lambda_rad));
    let ra_rad = libm::atan2(cos(epsilon_rad) * sin(lambda_rad), cos(lambda_rad));

    let gmst = normalize_degrees(280.46061837 + 360.98564736629 * n);

    let ha_deg = normalize_degrees(gmst + lon_deg - ra_rad * RAD_TO_DEG);
    let ha_rad = ha_deg * DEG_TO_RAD;

    let lat_rad = lat_deg * DEG_TO_RAD;

    let sin_elev = sin(lat_rad) * sin(decl_rad) + cos(lat_rad) * cos(decl_rad) * cos(ha_rad);
    let elevation_rad = asin(clamp_f64(sin_elev, -1.0, 1.0));
    let elevation_deg = elevation_rad * RAD_TO_DEG;

    let cos_elev = cos(elevation_rad);
    let azimuth_deg = if cos_elev.abs() < 1e-10 {
        180.0
    } else {
        let cos_az =
            (sin(decl_rad) - sin(lat_rad) * sin(elevation_rad)) / (cos(lat_rad) * cos_elev);
        let az_from_south = acos(clamp_f64(cos_az, -1.0, 1.0)) * RAD_TO_DEG;

        if ha_deg > 0.0 && ha_deg < 180.0 {
            180.0 + az_from_south
        } else {
            180.0 - az_from_south
        }
    };

    SunPosition {
        elevation_deg,
        azimuth_deg,
    }
}

/// TODO: Calculate the optimal panel tilt angle for the current sun position.
///
/// HINT: The ideal tilt for maximum direct irradiance is (90 - elevation).
/// Clamp the result to the actuator's physical range (TILT_MIN_DEG..TILT_MAX_DEG).
/// Return None if the sun is below DAYLIGHT_ELEVATION_DEG (night = use seasonal).
/// Return Some(TILT_MAX_DEG) if the sun is between -6 deg and 0 deg (twilight).
pub fn optimal_tilt(sun: &SunPosition) -> Option<f64> {
    // TODO: Implement this function
    // Step 1: Check if sun is below DAYLIGHT_ELEVATION_DEG → return None
    // Step 2: Check if sun is in twilight (0 to -6 deg) → return Some(TILT_MAX_DEG)
    // Step 3: Calculate ideal tilt = 90 - elevation
    // Step 4: Clamp to [TILT_MIN_DEG, TILT_MAX_DEG]
    todo!("Implement optimal_tilt — see hints above")
}

/// TODO: Seasonal default tilt angle when sun position is unavailable.
///
/// HINT: Use a cosine curve to interpolate between summer (8 deg, day 172)
/// and winter (24 deg, day 355). The formula is:
///   midpoint = (MIN + MAX) / 2
///   amplitude = (MAX - MIN) / 2
///   tilt = midpoint - amplitude * cos((day - 172) * 2*PI / 365)
///
/// Think about why we SUBTRACT the cosine (not add): at day 172, cos(0)=1,
/// and we want the MINIMUM tilt for summer.
pub fn seasonal_default(day_of_year: u32) -> f64 {
    // TODO: Implement this function
    // Step 1: Calculate midpoint and amplitude from TILT_MIN_DEG and TILT_MAX_DEG
    // Step 2: Compute cosine phase from day_of_year (offset by 172 = summer solstice)
    // Step 3: tilt = midpoint - amplitude * cos(phase)
    // Step 4: Clamp result to valid range
    todo!("Implement seasonal_default — see hints above")
}

/// Clamp a tilt angle to the actuator's physical range.
fn clamp_tilt(angle: f64) -> f64 {
    if angle < config::TILT_MIN_DEG {
        config::TILT_MIN_DEG
    } else if angle > config::TILT_MAX_DEG {
        config::TILT_MAX_DEG
    } else {
        angle
    }
}

fn normalize_degrees(deg: f64) -> f64 {
    let r = libm::fmod(deg, 360.0);
    if r < 0.0 { r + 360.0 } else { r }
}

fn clamp_f64(val: f64, min: f64, max: f64) -> f64 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

/// Calculate the day of year (1-366) from a Unix timestamp.
pub fn day_of_year(unix_secs: i64) -> u32 {
    let days = floor(unix_secs as f64 / 86400.0) as i64;
    let mut year = 1970i64;
    let mut remaining = days;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }

    (remaining + 1) as u32
}

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, tolerance: f64) -> bool {
        (a - b).abs() < tolerance
    }

    #[test]
    fn test_seasonal_default_summer() {
        let tilt = seasonal_default(172);
        assert!(
            approx_eq(tilt, 8.0, 0.5),
            "Summer solstice tilt should be ~8 deg, got {tilt}"
        );
    }

    #[test]
    fn test_seasonal_default_winter() {
        let tilt = seasonal_default(355);
        assert!(
            approx_eq(tilt, 24.0, 0.5),
            "Winter solstice tilt should be ~24 deg, got {tilt}"
        );
    }

    #[test]
    fn test_seasonal_default_equinox() {
        let tilt = seasonal_default(80);
        assert!(
            approx_eq(tilt, 16.0, 2.0),
            "Spring equinox tilt should be ~16 deg, got {tilt}"
        );
    }

    #[test]
    fn test_optimal_tilt_high_sun() {
        let pos = SunPosition {
            elevation_deg: 63.0,
            azimuth_deg: 180.0,
        };
        let tilt = optimal_tilt(&pos).unwrap();
        assert_eq!(tilt, 24.0, "High sun (63 deg) should clamp to 24 deg");
    }

    #[test]
    fn test_optimal_tilt_midrange() {
        let pos = SunPosition {
            elevation_deg: 75.0,
            azimuth_deg: 180.0,
        };
        let tilt = optimal_tilt(&pos).unwrap();
        assert!(
            approx_eq(tilt, 15.0, 0.01),
            "75 deg sun should give 15 deg tilt, got {tilt}"
        );
    }

    #[test]
    fn test_optimal_tilt_night() {
        let pos = SunPosition {
            elevation_deg: -10.0,
            azimuth_deg: 0.0,
        };
        assert!(optimal_tilt(&pos).is_none(), "Night should return None");
    }

    #[test]
    fn test_optimal_tilt_twilight() {
        let pos = SunPosition {
            elevation_deg: -3.0,
            azimuth_deg: 90.0,
        };
        let tilt = optimal_tilt(&pos).unwrap();
        assert_eq!(tilt, 24.0, "Twilight should give max tilt");
    }
}
