//! rs_solar_cntrl — ESP32-P4 Solar Controller (Free Edition)
//!
//! Manages a 12-panel TrinaSolar TSM-620NEG19RC.20 array on a wintergarden
//! roof with actuator-driven variable tilt (8 deg - 24 deg). Part of the
//! RISCVML educational curriculum.
//!
//! ## Student Exercise
//!
//! This is the free scaffolded version. The architecture and types are
//! provided — your task is to implement the TODO stubs in each module.
//! Run `cargo test` to check your progress.
//!
//! For the full reference solution: https://Agrarobotics.com/training

mod actuator;
mod adc_monitor;
mod config;
mod ffi;
mod hal;
mod inclinometer;
mod safety;
mod scheduler;
mod sun_position;
mod telemetry_db;
mod types;

fn main() {
    println!("=== rs_solar_cntrl — Solar Controller (Free Edition) ===");
    println!(
        "Site: {:.1} N, {:.1} E (Bad Schwalbach, Germany)",
        config::SITE_LATITUDE_DEG,
        config::SITE_LONGITUDE_DEG
    );
    println!(
        "Array: {} panels, {:.2} kWp, {} strings",
        config::PANEL_COUNT,
        config::SYSTEM_WP / 1000.0,
        config::STRING_COUNT
    );
    println!(
        "Tilt range: {:.0} deg - {:.0} deg",
        config::TILT_MIN_DEG,
        config::TILT_MAX_DEG
    );
    println!();

    // Demo: sun position calculation (provided complete)
    demo_sun_position();

    println!();
    println!("Implement the TODO stubs and run `cargo test` to verify.");
    println!("For the full reference solution: https://Agrarobotics.com/training");
}

fn demo_sun_position() {
    println!("--- Sun Position Demo ---");

    // Summer solstice noon (2024-06-21 12:00 UTC)
    let summer_noon = 1718971200i64;
    let pos = sun_position::sun_position(
        summer_noon,
        config::SITE_LATITUDE_DEG,
        config::SITE_LONGITUDE_DEG,
    );
    println!(
        "Summer solstice noon: elevation={:.1} deg, azimuth={:.1} deg",
        pos.elevation_deg, pos.azimuth_deg
    );

    // Winter solstice noon (2024-12-21 12:00 UTC)
    let winter_noon = 1734782400i64;
    let pos = sun_position::sun_position(
        winter_noon,
        config::SITE_LATITUDE_DEG,
        config::SITE_LONGITUDE_DEG,
    );
    println!(
        "Winter solstice noon: elevation={:.1} deg, azimuth={:.1} deg",
        pos.elevation_deg, pos.azimuth_deg
    );
}
