//! SQLite Detection Database — Sound Event Color Mappings
//!
//! Shares riscvml_detect.db with other use cases.
//! Adds a `sound_led_colors` table mapping sound event types
//! to alert-level RGB LED colors.

pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct DetectDb {
    conn: rusqlite::Connection,
}

pub fn open_or_create(path: &str) -> DetectDb {
    let conn = rusqlite::Connection::open(path)
        .expect("Failed to open riscvml_detect.db");

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS sound_led_colors (
            event   TEXT PRIMARY KEY,
            r       INTEGER NOT NULL,
            g       INTEGER NOT NULL,
            b       INTEGER NOT NULL
        );
        INSERT OR IGNORE INTO sound_led_colors (event, r, g, b) VALUES
            ('GlassBreak', 255, 23,  68 ),
            ('Doorbell',   255, 255, 255),
            ('DogBark',    255, 145, 0  ),
            ('Alarm',      255, 0,   0  ),
            ('Speech',     0,   176, 255),
            ('Unknown',    128, 128, 128);",
    ).expect("Failed to create sound_led_colors schema");

    DetectDb { conn }
}

impl DetectDb {
    pub fn get_led_color(&self, event: &str) -> RgbColor {
        self.conn
            .query_row(
                "SELECT r, g, b FROM sound_led_colors WHERE event = ?1",
                rusqlite::params![event],
                |row| Ok(RgbColor { r: row.get(0)?, g: row.get(1)?, b: row.get(2)? }),
            )
            .unwrap_or(RgbColor { r: 128, g: 128, b: 128 })
    }
}
