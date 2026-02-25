//! ESP32-C6 Companion — Wi-Fi 6 via ESP-Hosted/SDIO
//!
//! The ESP32-P4 has no built-in radio. A companion ESP32-C6 provides
//! Wi-Fi 6 connectivity via the ESP-Hosted protocol over SDIO. This
//! module manages the SDIO communication link and provides a network
//! interface for MQTT alerts and RTSP streaming.
//!
//! This architecture demonstrates the P4's intended companion-chip
//! design pattern: high-performance compute (P4) + wireless (C6).

use esp_idf_sys as _;

pub struct CompanionConfig {
    pub sdio_clock_hz: u32,
    pub wifi_ssid: heapless::String<32>,
    pub wifi_password: heapless::String<64>,
}

/// Handle to the ESP32-C6 companion chip connection.
pub struct C6Companion {
    _sdio_handle: *mut core::ffi::c_void,
    connected: bool,
}

/// Initialize the SDIO link to the ESP32-C6 companion.
pub fn init(_peripherals: &esp_hal::peripherals::Peripherals) -> C6Companion {
    // TODO: Initialize SDIO host interface on P4
    // TODO: Establish ESP-Hosted protocol handshake with C6
    // TODO: Wait for C6 Wi-Fi connection (STA mode)
    // TODO: Obtain IP address via DHCP

    defmt::info!("ESP32-C6 companion initialized via SDIO");

    todo!("companion initialization")
}

impl C6Companion {
    /// Stream an encoded H.264 chunk to the network.
    ///
    /// Used for both RTSP live streaming and MQTT event clips.
    /// Data flows: P4 encoder → SDIO → C6 → Wi-Fi 6 → network.
    pub async fn stream_chunk(&mut self, chunk: &super::encoder::EncodedChunk) {
        // TODO: Send encoded data over SDIO to C6
        // TODO: C6 forwards via RTSP server or MQTT publish
        todo!("stream chunk")
    }

    /// Send an MQTT alert when a bird is detected.
    pub async fn send_mqtt_alert(&mut self, detection: &crate::Detection) {
        // TODO: Serialize detection to JSON
        // TODO: Publish to MQTT topic "riscvml/detections"
        defmt::info!("MQTT alert: {}", detection.species.as_str());
        todo!("MQTT alert")
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}
