//! MQTT Alerts + RTSP Streaming — Network Services
//!
//! High-level network services built on top of the ESP32-C6 companion
//! link. Provides MQTT detection alerts and RTSP live video streaming
//! to remote clients.

/// MQTT topic for publishing bird detection events.
pub const MQTT_TOPIC_DETECTIONS: &str = "riscvml/bird-detection/detections";

/// MQTT topic for system status (heartbeat, battery, temperature).
pub const MQTT_TOPIC_STATUS: &str = "riscvml/bird-detection/status";

/// RTSP server port for live H.264 streaming.
pub const RTSP_PORT: u16 = 8554;

/// MQTT detection event payload.
///
/// Serialized as JSON and published to the detections topic each time
/// the inference engine identifies a bird.
///
/// Example payload:
/// ```json
/// {
///   "species": "European Robin",
///   "confidence": 0.94,
///   "bbox": { "x": 320, "y": 180, "w": 120, "h": 95 },
///   "timestamp": 1735689600000,
///   "led_color": { "r": 255, "g": 69, "b": 0 }
/// }
/// ```
pub struct MqttDetectionEvent<'a> {
    pub species: &'a str,
    pub confidence: f32,
    pub bbox: &'a crate::BoundingBox,
    pub timestamp_ms: u64,
    pub led_color: crate::detect_db::RgbColor,
}

/// Serialize a detection event to a compact JSON byte buffer.
pub fn serialize_detection_event(
    event: &MqttDetectionEvent,
    buf: &mut [u8],
) -> usize {
    // Manual JSON serialization — no serde dependency on embedded
    let s = core::fmt::write(
        &mut SliceWriter { buf, pos: 0 },
        format_args!(
            r#"{{"species":"{}","confidence":{:.2},"bbox":{{"x":{},"y":{},"w":{},"h":{}}},"timestamp":{},"led_color":{{"r":{},"g":{},"b":{}}}}}"#,
            event.species,
            event.confidence,
            event.bbox.x, event.bbox.y, event.bbox.w, event.bbox.h,
            event.timestamp_ms,
            event.led_color.r, event.led_color.g, event.led_color.b,
        ),
    );
    match s {
        Ok(()) => buf.len(), // full buffer used (approximate)
        Err(_) => 0,
    }
}

/// Helper for writing formatted text into a fixed-size byte slice.
struct SliceWriter<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> core::fmt::Write for SliceWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        if self.pos + bytes.len() > self.buf.len() {
            return Err(core::fmt::Error);
        }
        self.buf[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
        self.pos += bytes.len();
        Ok(())
    }
}
