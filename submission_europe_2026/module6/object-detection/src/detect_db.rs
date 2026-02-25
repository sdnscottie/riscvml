//! SQLite Detection Database — Object Detection Color Mappings
//!
//! Shares riscvml_detect.db with the bird detection capstone.
//! Adds an `object_led_colors` table mapping object classes to
//! threat-level RGB LED colors.

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
        "CREATE TABLE IF NOT EXISTS object_led_colors (
            class   TEXT PRIMARY KEY,
            r       INTEGER NOT NULL,
            g       INTEGER NOT NULL,
            b       INTEGER NOT NULL
        );
        INSERT OR IGNORE INTO object_led_colors (class, r, g, b) VALUES
            ('Person',   255, 234, 0  ),
            ('Vehicle',  0,   229, 255),
            ('Animal',   213, 0,   249),
            ('Bicycle',  0,   230, 118),
            ('Unknown',  128, 128, 128);",
    ).expect("Failed to create object_led_colors schema");

    DetectDb { conn }
}

impl DetectDb {
    pub fn get_led_color(&self, class: &str) -> RgbColor {
        self.conn
            .query_row(
                "SELECT r, g, b FROM object_led_colors WHERE class = ?1",
                rusqlite::params![class],
                |row| Ok(RgbColor { r: row.get(0)?, g: row.get(1)?, b: row.get(2)? }),
            )
            .unwrap_or(RgbColor { r: 128, g: 128, b: 128 })
    }
}
