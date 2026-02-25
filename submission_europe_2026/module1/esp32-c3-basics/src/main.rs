//! RISCVML Module 1: ESP32-C3 Basics
//!
//! Entry-level Rust embedded development on the ESP32-C3 (~€3).
//! Covers: Rust fundamentals, GPIO, sensors (I2C/SPI), BLE 5.0.
//!
//! Hardware: ESP32-C3 — Single-core 160 MHz RISC-V, 400 KB SRAM, BLE 5.0
//!
//! This module teaches:
//!   - Rust ownership, borrowing, and lifetimes in embedded context
//!   - GPIO pin configuration and digital I/O
//!   - I2C/SPI sensor communication (temperature, humidity, IMU)
//!   - BLE 5.0 peripheral advertising and GATT services
//!   - Basic interrupt handling and timer configuration

#![no_std]
#![no_main]

use esp_hal::prelude::*;

#[entry]
fn main() -> ! {
    defmt::info!("RISCVML Module 1 — ESP32-C3 Basics");

    let peripherals = esp_hal::init(esp_hal::Config::default());

    // TODO: GPIO LED blink — Chapter 1.1
    // TODO: Button input with interrupt — Chapter 1.2
    // TODO: I2C temperature sensor read — Chapter 1.3
    // TODO: SPI display output — Chapter 1.4
    // TODO: BLE advertising — Chapter 1.5

    loop {
        // Main application loop
    }
}
