//! RISCVML Module 3: ESP32-C6 Connectivity
//!
//! Wireless protocols and multi-device communication on ESP32-C6 (~€4).
//! Covers: Wi-Fi 6, BLE 5.3, ESP-NOW, MQTT, power management.
//!
//! Hardware: ESP32-C6 — Single-core 160 MHz RISC-V, Wi-Fi 6, BLE 5.3,
//!           Zigbee/Thread/Matter support
//!
//! This module teaches:
//!   - Wi-Fi 6 station and AP modes
//!   - ESP-NOW peer-to-peer communication
//!   - MQTT client for IoT cloud connectivity
//!   - Power management: deep sleep, light sleep, wake sources
//!   - Thread/Matter protocol basics for smart home integration

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use esp_hal::prelude::*;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("RISCVML Module 3 — ESP32-C6 Connectivity");

    let peripherals = esp_hal::init(esp_hal::Config::default());

    // TODO: Wi-Fi 6 STA connection — Chapter 3.1
    // TODO: ESP-NOW broadcast/receive — Chapter 3.2
    // TODO: MQTT publish/subscribe — Chapter 3.3
    // TODO: Deep sleep with timer wake — Chapter 3.4
    // TODO: Matter device setup — Chapter 3.5

    loop {
        embassy_time::Timer::after_secs(1).await;
    }
}
