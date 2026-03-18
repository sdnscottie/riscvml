# svbprj.md

This file provides guidance to CC (claude.ai/code) when working with code in this repository.

## What This Repo Is

A collection of **knowledge nuggets ("knuggets")** for the ESP32-P4 RISC-V MCU, part of the RISCVML curriculum. Each knugget is a self-contained learning unit demonstrating a specific capability of the ESP32-P4 hardware. The target board is the **Olimex ESP32-P4-PC** (16 MB flash, USB-Serial/JTAG on `/dev/ttyACM0`).

## Naming Convention

Directories use **double-underscore** separators: `esp32-p4__knuggets`, `esp32-p4__firststeps`, `esp32-p4__mlx90640`. Each knugget has a companion `docs_about__<knugget-name>/` subdirectory for documentation artifacts.

## Knuggets and Their Build Systems

### esp32-p4__firststeps — ESP-IDF C project (CMake)
```bash
# Activate ESP-IDF (required per terminal session)
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh

# Build
cd esp32-p4__firststeps/hello_p4
idf.py build

# Flash + Monitor (serial port: /dev/ttyACM0)
idf.py -p /dev/ttyACM0 flash
idf.py -p /dev/ttyACM0 monitor    # Ctrl+] to exit
```
- ESP-IDF v6.0-dev at `~/Dropbox/scottsoft_sdn/esp-idf/`
- Toolchain: `riscv32-esp-elf-gcc`
- `sdkconfig.defaults` sets target to `esp32p4` with 16 MB flash. If `sdkconfig` exists and is stale, delete it and rebuild.

### esp32-p4__mlx90640 — Rust thermal camera system (Cargo + ESP-IDF)

**Rust SPA Web Server** (axum):
```bash
cd esp32-p4__mlx90640

# Build (requires libudev-dev; workaround if not system-installed):
PKG_CONFIG_PATH=~/.local/lib/pkgconfig RUSTFLAGS="-L $HOME/.local/lib" cargo build

# Run with simulated data (no hardware needed):
cargo run -- --port 3030 --interval 5

# Run with real ESP32-P4 serial data:
cargo run -- --serial /dev/ttyACM0 --port 3030 --interval 5

# Debug serial separately:
rs_serialmon
```

**ESP-IDF Firmware** (C, CMake):
```bash
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh
cd esp32-p4__mlx90640/repo__esp32-p4__mlx90640__full/esp_idf_ws
idf.py build
idf.py -p /dev/ttyACM0 flash monitor   # Ctrl+] to exit
```

**Architecture:**
- MLX90640 32×24 IR sensor → I2C (0x33, 400kHz) → ESP32-P4 firmware → USB serial JSON → Rust web server → browser heatmap
- `src/main.rs`: axum server, save_frame (JSON+JPG), archive_if_needed (auto-archives after 5 frames)
- `src/sensor.rs`: ThermalFrame struct, simulated Gaussian heat blobs, parse_json_frame for serial input
- `src/serial.rs`: blocking serial reader with auto-reconnect (std::sync::RwLock, not tokio)
- `static/index.html`: canvas heatmap (640×480), polls /api/thermal every 2s, mouse hover shows pixel temps
- Results saved to `repo__esp32-p4__mlx90640__full/results/` (JSON + JPG), auto-archived to `results__archive/`
- Free/full repo pair: `repo__esp32-p4__mlx90640__free/` (student stubs) and `repo__esp32-p4__mlx90640__full/` (reference)

**Gotchas:**
- PlatformIO does NOT support ESP32-P4 — use ESP-IDF directly
- ESP32-P4 USB-Serial/JTAG = `/dev/ttyACM0` (not ttyUSB)
- `serialport` crate needs `libudev-dev` — use PKG_CONFIG_PATH workaround if not system-installed
- Use `std::sync::RwLock` (not tokio) for shared state between blocking serial thread and async web server
- Delete stale `sdkconfig` if changing ESP-IDF target
- ESP32-P4 has no built-in WiFi — data goes over USB serial
- Increase ESP-IDF stack size to 8192 to avoid stack overflow with MLX90640 calibration data

## ESP32-P4 Hardware Key Facts

- **No built-in Wi-Fi/Bluetooth** — uses companion chip (ESP32-C6) for wireless
- Dual-core RISC-V @ 400 MHz with custom `Xai` (ML/DSP) and `Xhwlp` (hardware loop) extensions
- MIPI-DSI display, MIPI-CSI camera with ISP, hardware H.264 encoder, JPEG codec
- USB 2.0 OTG High-Speed, 10/100 Ethernet MAC, 55 GPIOs
- The USB-Serial/JTAG peripheral enumerates as ACM (not ttyUSB)

## Detailed Reference

`esp32-p4__firststeps/svbprj.md` contains the comprehensive ESP32-P4 architecture reference, full setup walkthrough, common gotchas table, and VS Code integration notes.

## Commit Convention

Use `Co-Contributed-By: CC Opus 4.6 <noreply@anthropic.com>` in commits.
