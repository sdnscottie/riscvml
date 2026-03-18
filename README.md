# RISCVML

**Rust-first educational platform for RISC-V embedded systems and ML on Espressif hardware.**

172 chapters across 7 modules — from bare-metal basics on a €3 ESP32-C3 to on-device ML inference on the ESP32-P4.

**Author:** Scottie von Bruchhausen — [riscvml.org](https://riscvml.org)

## Hardware Ladder

| SoC | Role | Price | Key Features |
|-----|------|-------|-------------|
| **ESP32-C3** | Entry-level | ~€3 | Single-core 160 MHz RISC-V, BLE 5.0 |
| **ESP32-C6** | Connectivity | ~€4 | Wi-Fi 6, BLE 5.3, Zigbee/Thread, Matter-ready |
| **ESP32-P4** | High-performance | ~€25 | Dual-core 400 MHz, AI/vector extensions, MIPI-CSI/DSI, H.264, PPA |

## Repository Structure

```
riscvml/
├── esp32-p4_knuggets/          ESP32-P4 knowledge nuggets (hardware deep-dives)
│   ├── ...__firststeps/        First steps with ESP32-P4
│   ├── ...__motor_driver/      IBT-2 BTS7960 + PCA9685 servo control
│   ├── ...__secure_wap_streamer/  Secure Wi-Fi AP + camera streaming
│   ├── ...__ws_thermal_cam_mod/   MLX90640 thermal camera
│   └── ...__mlx90640/          Thermal camera (Rust)
├── riscvml_projects/           Curriculum exercise projects
│   ├── rs_riscvml__embedded_solar_cntrl/  12-panel solar tilt controller
│   └── rs_riscvml__embedded_mini_ups/     Mini UPS project
└── submission_europe_2026/     → separate repo (public)
```

Each knugget follows a **free/full** pattern:
- `__free` — student scaffolding with TODOs (public)
- `__full` — reference solution (private)

## Curriculum Modules

| Module | Topic | Hardware |
|--------|-------|----------|
| 1 | Rust fundamentals, GPIO, sensors | ESP32-C3 |
| 2 | I2C, PCA9685 servo driver, motor control | ESP32-C3/C6 |
| 3 | Power management, solar control | ESP32-C3 |
| 4 | LoRa, long-range communication | TTGO T-Beam |
| 5 | Multi-device: ESP-NOW, MQTT | ESP32-C6 |
| 6 | ESP32-P4 high-performance: ISP, PPA, H.264, AI/vector, DMA | ESP32-P4 |
| 7 | Firmware-to-desktop bridge via Tauri | All |

## Capstone: Bird Detection Pipeline (ESP32-P4)

End-to-end on-device ML system exercising every P4 subsystem:

1. **Capture** — MIPI-CSI camera → ISP pipeline → DMA → display (30–60 FPS)
2. **Detect** — esp-dl quantized inference → PPA bounding box overlay → SQLite logging
3. **React** — PCA9685 pan/tilt servo tracking → H.264 recording → MQTT alerts via companion ESP32-C6

Uses the reusable **Detect → Visualize → React** pattern:
```
Sensor → ML Classification → SQLite Lookup → RGB LED → Reaction
```

## Tech Stack

- **Language:** Rust (no_std + ESP-IDF via FFI)
- **Async runtime:** Embassy
- **ML inference:** esp-dl / esp-detection (quantized INT8)
- **Database:** SQLite3 (detection logging, telemetry)
- **Diagrams:** draw.io (`.drawio` + `.png` exports)

## Build

```bash
# Rust projects
cargo build
cargo clippy
cargo fmt -- --check

# ESP-IDF projects
source ~/esp-idf/export.sh
idf.py build
idf.py -p /dev/ttyACM0 flash monitor
```

## Conference

Submitted to **RISC-V Summit Europe 2026** (Bologna, June 8–12).
See [submission_europe_2026](https://github.com/sdnscottie/submission_europe_2026) repo.

## License

Educational content. Contact scottie@riscvml.org for licensing.
