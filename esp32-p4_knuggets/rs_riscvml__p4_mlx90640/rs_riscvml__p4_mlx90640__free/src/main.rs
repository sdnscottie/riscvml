mod sensor;
mod serial;
mod spa;

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use axum::{Router, extract::State, routing::get};
use clap::Parser;

use sensor::ThermalFrame;

const DEFAULT_RESULTS_DIR: &str = "repo__esp32-p4__mlx90640__full/results";

#[derive(Parser)]
#[command(name = "mlx90640-viewer")]
#[command(about = "MLX90640 Thermal Viewer — RISCVML")]
struct Cli {
    /// Serial port to read from (e.g. /dev/ttyACM0). Omit for simulated data.
    #[arg(short, long)]
    serial: Option<String>,

    /// Baud rate for serial connection
    #[arg(short, long, default_value_t = 115200)]
    baud: u32,

    /// HTTP server port
    #[arg(short, long, default_value_t = 3030)]
    port: u16,

    /// Directory to store captured thermal frames as JSON
    #[arg(long, default_value = DEFAULT_RESULTS_DIR)]
    results_dir: PathBuf,

    /// Capture interval in seconds (simulated mode; serial mode reads as fast as firmware sends)
    #[arg(short, long, default_value_t = 5)]
    interval: u64,
}

type SharedFrame = Arc<RwLock<ThermalFrame>>;

/// API handler: return the current thermal frame as JSON.
///
/// Hint: Use `frame.read().unwrap()` to acquire a read lock (std::sync::RwLock),
/// then clone the frame and wrap in `axum::Json`.
async fn thermal_api(State(_frame): State<SharedFrame>) -> axum::Json<ThermalFrame> {
    todo!("Exercise: Read the shared ThermalFrame via frame.read().unwrap() and return it as axum::Json")
}

/// Save a thermal frame to disk as JSON + JPG heatmap.
///
/// Hints:
/// - Create the results_dir if it doesn't exist (`std::fs::create_dir_all`)
/// - Derive a base filename from `frame.image_ts` (replace ':' with '_')
/// - Serialize the frame to pretty JSON with `serde_json::to_string_pretty`
/// - Write the JSON file to `{results_dir}/{base_name}.json`
/// - Call `save_heatmap_jpg` to write a visual heatmap as `{base_name}.jpg`
/// - Call `archive_if_needed` to rotate old frames
#[allow(dead_code)]
pub fn save_frame(_results_dir: &std::path::Path, _frame: &ThermalFrame) {
    todo!("Exercise: Save frame as JSON + JPG heatmap, then call archive_if_needed()")
}

/// Archive results when 5 or more JSON files accumulate.
///
/// Hints:
/// - Read the results_dir and count files with .json extension
/// - If fewer than 5, return early
/// - Create an archive directory by appending "__archive" to the results dir name
///   (e.g., "results" -> "results__archive")
/// - Move all files (json + jpg) from results_dir to the archive directory
/// - Print how many files were archived
#[allow(dead_code)]
fn archive_if_needed(_results_dir: &std::path::Path) {
    todo!("Exercise: Move all files to archive dir when >= 5 JSON frames exist")
}

/// Convert a normalized temperature (0.0..1.0) to an RGB color for heatmap rendering.
///
/// Hints:
/// - Clamp the input to 0.0..1.0
/// - Use a 4-stop gradient: Blue -> Cyan -> Green -> Yellow -> Red
///   - 0.00..0.25: Blue(0,0,255) to Cyan(0,255,255)  — interpolate green up
///   - 0.25..0.50: Cyan(0,255,255) to Green(0,255,0)  — interpolate blue down
///   - 0.50..0.75: Green(0,255,0) to Yellow(255,255,0) — interpolate red up
///   - 0.75..1.00: Yellow(255,255,0) to Red(255,0,0)   — interpolate green down
/// - Return [r, g, b] as u8 values
#[allow(dead_code)]
fn temp_to_rgb(_t: f32) -> [u8; 3] {
    todo!("Exercise: Map normalized 0..1 temperature to RGB using 4-stop color gradient")
}

/// Save a thermal frame as a scaled-up JPG heatmap image.
///
/// Hints:
/// - Use `image::ImageBuffer::new(w, h)` to create an RGB image
/// - Scale each sensor pixel to 20x20 output pixels (scale = 20)
/// - For each sensor pixel, normalize temp: `t = (temp - min_temp) / range`
/// - Use `temp_to_rgb(t)` to get the color, then `img.put_pixel(x, y, Rgb(rgb))`
/// - Save with `img.save(path)`
#[allow(dead_code)]
fn save_heatmap_jpg(_path: &std::path::Path, _frame: &ThermalFrame) {
    todo!("Exercise: Render thermal frame as a scaled JPG heatmap using the image crate")
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let results_dir = cli.results_dir.clone();

    let interval = cli.interval;
    let mut initial_frame = sensor::generate_simulated_frame();
    initial_frame.capture_interval_secs = interval;
    let shared_frame: SharedFrame = Arc::new(RwLock::new(initial_frame));

    // TODO: Spawn a data source based on whether --serial was provided.
    //
    // If cli.serial is Some(port_name):
    //   - Print which port and baud rate you're connecting to
    //   - Clone the shared_frame Arc and results_dir
    //   - Use tokio::task::spawn_blocking to run serial::serial_reader_loop()
    //     in a dedicated thread (it's a blocking function)
    //   - Pass &rdir, interval as extra arguments
    //
    // If cli.serial is None:
    //   - Print that you're using simulated data
    //   - Clone the shared_frame Arc and results_dir
    //   - Use tokio::spawn to run an async loop that:
    //     1. Generates a new frame with sensor::generate_simulated_frame()
    //     2. Sets frame.capture_interval_secs = interval
    //     3. Calls save_frame(&rdir, &frame)
    //     4. Acquires a write lock: frame_ref.write().unwrap()
    //     5. Replaces the shared frame
    //     6. Sleeps for `interval` seconds before repeating
    //
    // Note: Use std::sync::RwLock — `.write().unwrap()` (NOT `.write().await`)
    let _ = &cli.serial; // suppress unused warning — remove when implementing
    let _ = &results_dir; // suppress unused warning — remove when implementing
    let _ = interval; // suppress unused warning — remove when implementing

    let app = Router::new()
        .route("/", get(spa::index))
        .route("/api/thermal", get(thermal_api))
        .with_state(shared_frame);

    let addr = format!("0.0.0.0:{}", cli.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("MLX90640 Thermal Viewer running at http://localhost:{}", cli.port);
    println!("Frames saved to: {}", results_dir.display());
    println!("  Debug serial with: rs_serialmon");
    axum::serve(listener, app).await.unwrap();
}
