# svbprj.md

This file provides guidance to CC (claude.ai/code) when working with code in this repository.

## Project Overview

**rs_riscvml__esp32-p4-wifi6-kit-a__firststeps** — First steps knugget for the Waveshare ESP32-P4 Wi-Fi 6 Kit A development board. Part of the RISCVML educational curriculum (esp32-p4_knuggets).

## Board: Waveshare ESP32-P4 Wi-Fi 6 Kit A

| Feature | Detail |
|---------|--------|
| SoC | ESP32-P4 dual-core RISC-V 400 MHz HP + 40 MHz LP |
| Companion | ESP32-C6 (Wi-Fi 6 / BLE 5 / 802.15.4 Thread) via SDIO |
| Memory | 32 MB PSRAM, 16 MB Flash |
| I2C | SDA: GPIO7, SCL: GPIO8 (default) |
| I2S Audio | ES8311 codec (GPIO9–13, PA: GPIO53) |
| SDMMC | 4-wire TF card (GPIO39–44) |
| MIPI | CSI 2-lane camera, DSI 2-lane display |
| GPIO Header | 2×20 pins (RPi HAT compatible), 27 programmable GPIOs |
| USB UART (P4) | CH9103 → `/dev/ttyACM2` (PID: 55d3) — **flash here** |
| USB UART (C6) | CH9104 → `/dev/ttyACM1` (PID: 55d4) — NOT the P4! |
| USB OTG | Native ESP32-P4 USB 2.0 HS |

## Repositories

| Repo | Visibility | URL |
|------|-----------|-----|
| **riscvml** (monorepo) | Public | https://github.com/sdnscottie/riscvml |
| **__free** (student exercise) | Public | https://github.com/sdnscottie/rs_riscvml__esp32-p4-wifi6-kit-a__firststeps__free |
| **__full** (reference solution) | Private | https://github.com/sdnscottie/rs_riscvml__esp32-p4-wifi6-kit-a__firststeps__full |

## Repository Layout: Free/Full Pattern

- `rs_riscvml__esp32-p4-wifi6-kit-a__firststeps__free/` — **Student exercise** (scaffolded, incomplete)
- `rs_riscvml__esp32-p4-wifi6-kit-a__firststeps__full/` — **Reference solution** (gold/complete)

When developing: implement features in `__full` first, then create the corresponding scaffolded version in `__free` with TODO markers or stubs for students to complete.

## Project Contents (__full)

### hello_world_p4/ — ESP-IDF C hello world
- Verified flashing and serial output on Waveshare ESP32-P4 Wi-Fi 6 Kit A
- `sdkconfig.defaults`: target=esp32p4, UART console, 32MB flash

### Rust SPA Serial Viewer — axum web server
- `src/main.rs` — axum + blocking serial reader thread + `Arc<RwLock<SerialBuffer>>`
- `src/serial.rs` — blocking serial reader with reconnect logic
- `src/spa.rs` — embedded HTML via `include_str!`
- `static/index.html` — terminal-style SPA with auto-scroll, log download, syntax highlighting
- Same architecture pattern as rs_riscvml__p4_mlx90640 thermal viewer

### docs_about__esp32-p4-wifi6-kit-a__firststeps__full/
- `firststeps_guide_and_gotchas.drawio` — 3-step guide + 4 gotchas
- `rust_spa_serial_viewer.drawio` — 3-layer architecture diagram
- `esp32_p4_wifi6_kit_a_pinout.drawio` — Board pinout & interfaces
- `thermal_camera_comparison.drawio` — MLX90640 vs Waveshare Thermal Camera Module (MI0802)

### Planned: Waveshare Thermal Camera Module (80×62)
- Meridian MI0802 sensor, I2C config + SPI data, 25 FPS
- Compare with MLX90640 (32×24, I2C only) from rs_riscvml__p4_mlx90640
- Same Rust SPA heatmap pattern: sensor → ESP32-P4 → JSON → axum → browser

## Build Commands

```bash
# Rust SPA Serial Viewer (in __full/ directory)
PKG_CONFIG_PATH=~/.local/lib/pkgconfig RUSTFLAGS="-L /home/maxx/.local/lib" cargo build
cargo run -- --serial /dev/ttyACM2 --baud 115200 --port 3030
cargo run  # simulated mode (no board needed)

# ESP-IDF hello world (in hello_world_p4/ directory)
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh
idf.py build
idf.py -p /dev/ttyACM2 flash         # flash to P4 (NOT ttyACM1!)
idf.py -p /dev/ttyACM2 flash monitor # Ctrl+] to exit monitor

# Rust projects (general)
cargo test
cargo clippy
cargo fmt -- --check
```

## Gotchas

1. **Wrong USB port**: ttyACM1 is the ESP32-C6 companion (CH9104), NOT the P4. Use ttyACM2 (CH9103).
2. **Console config**: Use `CONFIG_ESP_CONSOLE_UART_DEFAULT=y`, NOT `USB_SERIAL_JTAG`. Waveshare routes UART0 via CH9103.
3. **Flash size mismatch**: Add `CONFIG_ESPTOOLPY_FLASHSIZE_32MB=y` to sdkconfig.defaults.
4. **No onboard LED**: Unlike Olimex P4-PC, this board has no user-accessible LED for blink demos.
5. **libudev for serialport crate**: Build needs `PKG_CONFIG_PATH=~/.local/lib/pkgconfig RUSTFLAGS="-L /home/maxx/.local/lib"`.
6. **idf.py monitor needs TTY**: Use rs_serialmon or the Rust SPA viewer instead of `idf.py monitor` from non-TTY environments.

## Conventions

- Commits use: `Co-Contributed-By: CC Opus 4.6 <noreply@anthropic.com>`
- Follow RISCVML patterns: SQLite for telemetry, embedded-hal traits for hardware abstraction
- Diagrams go in `docs_about__esp32-p4-wifi6-kit-a__firststeps__[free|full]/` directories
- Regenerate diagram PNGs: `drawio --export --format png --scale 2 --output X.png X.drawio`
