use std::path::Path;
use std::sync::{Arc, RwLock};

use crate::sensor::ThermalFrame;

/// Blocking serial reader loop — runs in a dedicated thread.
/// Reads JSON lines from the ESP32-P4 firmware and updates the shared frame.
///
/// Hints:
/// - Loop forever, reconnecting on errors (add `use std::time::Duration;`, `use std::io::BufRead;`)
/// - Use `serialport::new(port_name, baud_rate).timeout(Duration::from_secs(30)).open()` to open
/// - Wrap the port in a `BufReader` and read lines
/// - Skip non-JSON lines (ESP-IDF boot logs); only parse lines starting with '{'
/// - Use `crate::sensor::parse_json_frame()` to convert JSON lines into ThermalFrame
/// - On successful parse: set `parsed.capture_interval_secs = interval`
/// - Call `crate::save_frame(results_dir, &parsed)` to persist the frame
/// - Update the shared frame: `frame.write().unwrap()` (std::sync::RwLock, not tokio)
/// - Handle timeout errors gracefully (sensor may send every 15s, timeout is 30s)
/// - On errors (EOF, read error, open failure), sleep 3 seconds and retry
pub fn serial_reader_loop(
    _port_name: &str,
    _baud_rate: u32,
    _frame: Arc<RwLock<ThermalFrame>>,
    _results_dir: &Path,
    _interval: u64,
) {
    todo!("Exercise: Implement serial port reader that parses JSON thermal frames")
}
