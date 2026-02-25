//! SQLite Detection Database — Plant Health Color Mappings
//!
//! Shares riscvml_detect.db with other use cases.
//! Adds a `health_led_colors` table mapping plant health status
//! to intuitive RGB LED colors.

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
        "CREATE TABLE IF NOT EXISTS health_led_colors (
            status  TEXT PRIMARY KEY,
            r       INTEGER NOT NULL,
            g       INTEGER NOT NULL,
            b       INTEGER NOT NULL
        );
        INSERT OR IGNORE INTO health_led_colors (status, r, g, b) VALUES
            ('Healthy',            0,   230, 118),
            ('Stressed',           255, 234, 0  ),
            ('NutrientDeficient',  255, 152, 0  ),
            ('Diseased',           255, 23,  68 ),
            ('Unknown',            128, 128, 128);",
    ).expect("Failed to create health_led_colors schema");

    DetectDb { conn }
}

impl DetectDb {
    pub fn get_led_color(&self, status: &str) -> RgbColor {
        self.conn
            .query_row(
                "SELECT r, g, b FROM health_led_colors WHERE status = ?1",
                rusqlite::params![status],
                |row| Ok(RgbColor { r: row.get(0)?, g: row.get(1)?, b: row.get(2)? }),
            )
            .unwrap_or(RgbColor { r: 128, g: 128, b: 128 })
    }
}
