//! SQLite telemetry database for solar controller data.
//!
//! ## Your Task
//!
//! Implement the INSERT methods for each table:
//! - `insert_power_reading()`
//! - `insert_tilt_event()`
//! - `insert_status_snapshot()`
//! - `insert_safety_event()`
//! - `upsert_daily_summary()`
//!
//! The schema creation and query methods are provided.
//! Use the unit tests as your specification.
//!
//! For the full reference solution, visit: https://Agrarobotics.com/training

use rusqlite::{Connection, params};

use crate::types::{PowerReading, SolarError, SystemStatus, TiltReason};

pub struct TelemetryDb {
    conn: Connection,
}

impl TelemetryDb {
    pub fn open(path: &str) -> Result<Self, SolarError> {
        let conn = Connection::open(path)?;
        let db = TelemetryDb { conn };
        db.create_tables()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self, SolarError> {
        let conn = Connection::open_in_memory()?;
        let db = TelemetryDb { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<(), SolarError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS power_readings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                string_id INTEGER NOT NULL,
                voltage_v REAL NOT NULL,
                current_a REAL NOT NULL,
                power_w REAL NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tilt_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                from_deg REAL NOT NULL,
                to_deg REAL NOT NULL,
                reason TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS status_snapshots (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                tilt_deg REAL NOT NULL,
                total_power_w REAL NOT NULL,
                string_a_v REAL, string_a_a REAL, string_a_w REAL,
                string_b_v REAL, string_b_a REAL, string_b_w REAL,
                string_c_v REAL, string_c_a REAL, string_c_w REAL,
                wind_lockout INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS safety_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                event_type TEXT NOT NULL,
                detail TEXT
            );

            CREATE TABLE IF NOT EXISTS daily_summary (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL UNIQUE,
                total_kwh REAL NOT NULL,
                peak_power_w REAL NOT NULL,
                tilt_changes INTEGER NOT NULL,
                min_tilt_deg REAL,
                max_tilt_deg REAL
            );

            CREATE INDEX IF NOT EXISTS idx_power_ts ON power_readings(timestamp);
            CREATE INDEX IF NOT EXISTS idx_tilt_ts ON tilt_events(timestamp);
            CREATE INDEX IF NOT EXISTS idx_status_ts ON status_snapshots(timestamp);
            CREATE INDEX IF NOT EXISTS idx_safety_ts ON safety_events(timestamp);
            CREATE INDEX IF NOT EXISTS idx_daily_date ON daily_summary(date);",
        )?;
        Ok(())
    }

    /// TODO: Insert a power reading from one string.
    pub fn insert_power_reading(&self, reading: &PowerReading) -> Result<(), SolarError> {
        // TODO: INSERT INTO power_readings (timestamp, string_id, voltage_v, current_a, power_w)
        // Use self.conn.execute() with params![]
        todo!("Implement insert_power_reading")
    }

    /// TODO: Record a tilt position change.
    pub fn insert_tilt_event(
        &self,
        timestamp: i64,
        from_deg: f64,
        to_deg: f64,
        reason: TiltReason,
    ) -> Result<(), SolarError> {
        // TODO: Convert TiltReason to a string, then INSERT INTO tilt_events
        // TiltReason::SunTracking → "sun_tracking", etc.
        todo!("Implement insert_tilt_event")
    }

    /// TODO: Insert a full system status snapshot.
    pub fn insert_status_snapshot(&self, status: &SystemStatus) -> Result<(), SolarError> {
        // TODO: INSERT INTO status_snapshots with all 13 columns
        todo!("Implement insert_status_snapshot")
    }

    /// TODO: Record a safety event.
    pub fn insert_safety_event(
        &self,
        timestamp: i64,
        event_type: &str,
        detail: Option<&str>,
    ) -> Result<(), SolarError> {
        // TODO: INSERT INTO safety_events (timestamp, event_type, detail)
        todo!("Implement insert_safety_event")
    }

    /// TODO: Upsert a daily summary row.
    pub fn upsert_daily_summary(
        &self,
        date: &str,
        total_kwh: f64,
        peak_power_w: f64,
        tilt_changes: i32,
        min_tilt_deg: f64,
        max_tilt_deg: f64,
    ) -> Result<(), SolarError> {
        // TODO: INSERT ... ON CONFLICT(date) DO UPDATE SET ...
        todo!("Implement upsert_daily_summary")
    }

    pub fn get_daily_kwh(&self, date: &str) -> Result<Option<f64>, SolarError> {
        let result = self.conn.query_row(
            "SELECT total_kwh FROM daily_summary WHERE date = ?1",
            params![date],
            |row| row.get(0),
        );
        match result {
            Ok(kwh) => Ok(Some(kwh)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::StringStatus;

    #[test]
    fn test_create_database() {
        let db = TelemetryDb::open_in_memory().unwrap();
        db.insert_safety_event(1000, "test", Some("init")).unwrap();
    }

    #[test]
    fn test_insert_power_reading() {
        let db = TelemetryDb::open_in_memory().unwrap();
        let reading = PowerReading {
            string_id: 0,
            voltage_v: 162.3,
            current_a: 14.5,
            power_w: 2353.35,
            timestamp: 1718971200,
        };
        db.insert_power_reading(&reading).unwrap();

        let count: i64 = db
            .conn
            .query_row("SELECT COUNT(*) FROM power_readings", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_insert_tilt_event() {
        let db = TelemetryDb::open_in_memory().unwrap();
        db.insert_tilt_event(1000, 16.0, 12.0, TiltReason::SunTracking)
            .unwrap();

        let reason: String = db
            .conn
            .query_row("SELECT reason FROM tilt_events WHERE id = 1", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(reason, "sun_tracking");
    }

    #[test]
    fn test_daily_summary_upsert() {
        let db = TelemetryDb::open_in_memory().unwrap();
        db.upsert_daily_summary("2024-06-21", 35.2, 6800.0, 12, 8.0, 20.0)
            .unwrap();

        let kwh = db.get_daily_kwh("2024-06-21").unwrap();
        assert_eq!(kwh, Some(35.2));

        db.upsert_daily_summary("2024-06-21", 38.5, 7100.0, 15, 8.0, 22.0)
            .unwrap();
        let kwh = db.get_daily_kwh("2024-06-21").unwrap();
        assert_eq!(kwh, Some(38.5));
    }
}
