//! RISCVML Module 6: Sound/Voice Classification
//!
//! Detect → Visualize → React pattern applied to audio events.
//! I2S/PDM microphone captures audio, ML classifies sound type,
//! RGB LED shows event category color.
//!
//! Pipeline: I2S/PDM Microphone → ML Classification → SQLite lookup
//!           → RGB LED (event color) → Security alert / audio record

#![no_std]
#![no_main]

mod detect_db;
mod led;

use embassy_executor::Spawner;
use esp_hal::prelude::*;

pub enum SoundClass {
    GlassBreak,
    Doorbell,
    DogBark,
    Alarm,
    Speech,
    Unknown,
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("RISCVML Sound Classification — Detect → Visualize → React");

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let db = detect_db::open_or_create("/sdcard/riscvml_detect.db");
    let led = led::init(&peripherals);

    // TODO: Initialize I2S microphone (PDM or standard I2S)
    // TODO: Configure ring buffer for 1-second audio windows
    // TODO: Load audio classification model (keyword spotting / YAMNet)
    // TODO: Run Detect → Visualize → React loop
    //   - Classify sounds: Glass break (red), Doorbell (white), Dog bark (orange)
    //   - Set RGB LED to event category color
    //   - Record audio clip and send security alert via MQTT

    loop {
        embassy_time::Timer::after_millis(100).await; // 10 Hz classification
    }
}
