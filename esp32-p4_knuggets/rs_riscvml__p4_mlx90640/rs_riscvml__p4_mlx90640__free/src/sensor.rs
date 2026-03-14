use serde::Serialize;

pub const ROWS: usize = 24;
pub const COLS: usize = 32;

#[derive(Serialize, Clone)]
pub struct ThermalFrame {
    pub rows: usize,
    pub cols: usize,
    pub pixels: Vec<f32>,
    pub min_temp: f32,
    pub max_temp: f32,
    pub avg_temp: f32,
    pub unit: &'static str,
    pub image_ts: String,
    pub capture_interval_secs: u64,
}

/// Helper struct for placing simulated heat sources.
/// Each blob is a Gaussian "warm spot" with a center, peak temperature offset, and spread.
#[allow(dead_code)]
struct HeatBlob {
    cx: f32,
    cy: f32,
    peak: f32,
    sigma: f32,
}

/// Generate a simulated 32x24 thermal frame with realistic heat patterns.
///
/// Hints:
/// - Use `rand::rng()` to get a random number generator (add `use rand::Rng;`)
/// - Create a baseline temperature (e.g., 22.0 C)
/// - Generate 2-3 random HeatBlob structs with random positions, peaks, and sigmas
///   - cx: 4.0..28.0, cy: 4.0..20.0, peak: 8.0..15.0, sigma: 3.0..6.0
/// - For each pixel (row, col), start with baseline + small noise (-0.3..0.3)
/// - Add Gaussian contribution from each blob: `peak * exp(-dist^2 / (2 * sigma^2))`
/// - Round each pixel to 1 decimal place: `(temp * 10.0).round() / 10.0`
/// - Compute min_temp, max_temp, avg_temp from the pixel array
/// - You'll need a timestamp — implement a `now_iso8601()` helper that formats
///   `SystemTime::now()` as "YYYY-MM-DDThh:mm:ss.mmmZ" (use the civil_from_days algorithm)
/// - Return a ThermalFrame with rows=ROWS, cols=COLS, unit="celsius",
///   capture_interval_secs=5
pub fn generate_simulated_frame() -> ThermalFrame {
    todo!("Exercise: Generate simulated thermal data with heat blobs")
}

/// Parse a JSON line from the ESP32-P4 firmware into a ThermalFrame.
///
/// Expected JSON format from firmware:
/// `{"rows":24,"cols":32,"vdd":3.3,"ta":25.1,"pixels":[t0,t1,...,t767]}`
///
/// Hints:
/// - Parse the JSON string using `serde_json::from_str::<serde_json::Value>()`
/// - Extract the "pixels" array and convert each element to f32
/// - Verify the pixel count equals ROWS * COLS, return None if not
/// - Compute min_temp, max_temp, avg_temp from the pixels
/// - Generate a timestamp with `now_iso8601()`
/// - Return `Some(ThermalFrame { ... })` with unit="celsius", capture_interval_secs=5
pub fn parse_json_frame(_json: &str) -> Option<ThermalFrame> {
    todo!("Exercise: Parse JSON frame from ESP32-P4 serial output")
}
