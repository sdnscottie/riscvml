//! SQLite Detection Database — riscvml_detect.db
//!
//! Persists detection events and species→RGB color mappings to a SQLite
//! database on the SD card. Two tables:
//!
//!   `detections`      — species, confidence, bounding box, timestamp
//!   `bird_led_colors` — species name → RGB LED color mapping
//!
//! This module implements the "data logging" step of the
//! Detect → Visualize → React pattern.

/// RGB color value for LED output.
#[derive(Clone, Copy, Debug)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Handle to the opened riscvml_detect.db database.
pub struct DetectDb {
    conn: rusqlite::Connection,
}

/// Open or create the detection database with both tables.
///
/// Creates the schema on first run:
/// ```sql
/// CREATE TABLE IF NOT EXISTS detections (
///     id         INTEGER PRIMARY KEY AUTOINCREMENT,
///     species    TEXT NOT NULL,
///     confidence REAL NOT NULL,
///     bbox_x     INTEGER,
///     bbox_y     INTEGER,
///     bbox_w     INTEGER,
///     bbox_h     INTEGER,
///     timestamp  INTEGER NOT NULL
/// );
///
/// CREATE TABLE IF NOT EXISTS bird_led_colors (
///     species TEXT PRIMARY KEY,
///     r       INTEGER NOT NULL,
///     g       INTEGER NOT NULL,
///     b       INTEGER NOT NULL
/// );
/// ```
pub fn open_or_create(path: &str) -> DetectDb {
    let conn = rusqlite::Connection::open(path)
        .expect("Failed to open riscvml_detect.db");

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS detections (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            species    TEXT NOT NULL,
            confidence REAL NOT NULL,
            bbox_x     INTEGER,
            bbox_y     INTEGER,
            bbox_w     INTEGER,
            bbox_h     INTEGER,
            timestamp  INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS bird_led_colors (
            species TEXT PRIMARY KEY,
            r       INTEGER NOT NULL,
            g       INTEGER NOT NULL,
            b       INTEGER NOT NULL
        );
        -- Seed default color mappings
        INSERT OR IGNORE INTO bird_led_colors (species, r, g, b) VALUES
            ('European Robin',    255, 69,  0  ),
            ('Blue Tit',          0,   120, 255),
            ('Great Tit',         255, 215, 0  ),
            ('House Sparrow',     139, 90,  43 ),
            ('Blackbird',         30,  30,  30 ),
            ('Goldfinch',         255, 200, 0  ),
            ('Woodpecker',        200, 0,   0  ),
            ('Magpie',            0,   0,   0  ),
            ('Jay',               180, 130, 200),
            ('Starling',          50,  180, 100),
            ('Unknown',           128, 128, 128);",
    ).expect("Failed to create schema");

    defmt::info!("Opened riscvml_detect.db at {}", path);

    DetectDb { conn }
}

impl DetectDb {
    /// Insert a detection event into the `detections` table.
    pub fn insert_detection(&self, det: &crate::Detection) {
        self.conn.execute(
            "INSERT INTO detections (species, confidence, bbox_x, bbox_y, bbox_w, bbox_h, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                det.species.as_str(),
                det.confidence,
                det.bbox.x,
                det.bbox.y,
                det.bbox.w,
                det.bbox.h,
                det.timestamp_ms,
            ],
        ).expect("Failed to insert detection");
    }

    /// Look up the RGB LED color for a detected species.
    ///
    /// Falls back to the 'Unknown' color (gray) if the species isn't
    /// in the bird_led_colors table.
    pub fn get_led_color(&self, species: &str) -> RgbColor {
        self.conn
            .query_row(
                "SELECT r, g, b FROM bird_led_colors WHERE species = ?1",
                rusqlite::params![species],
                |row| {
                    Ok(RgbColor {
                        r: row.get(0)?,
                        g: row.get(1)?,
                        b: row.get(2)?,
                    })
                },
            )
            .unwrap_or(RgbColor { r: 128, g: 128, b: 128 })
    }
}
