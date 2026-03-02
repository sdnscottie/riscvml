# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

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

### esp32-p4__mlx90640 — Rust project (Cargo)
```bash
cd esp32-p4__mlx90640
cargo build
```
- Edition 2024, currently a stub. Will target the MLX90640 thermal camera sensor.

## ESP32-P4 Hardware Key Facts

- **No built-in Wi-Fi/Bluetooth** — uses companion chip (ESP32-C6) for wireless
- Dual-core RISC-V @ 400 MHz with custom `Xai` (ML/DSP) and `Xhwlp` (hardware loop) extensions
- MIPI-DSI display, MIPI-CSI camera with ISP, hardware H.264 encoder, JPEG codec
- USB 2.0 OTG High-Speed, 10/100 Ethernet MAC, 55 GPIOs
- The USB-Serial/JTAG peripheral enumerates as ACM (not ttyUSB)

## Detailed Reference

`esp32-p4__firststeps/svbprj.md` contains the comprehensive ESP32-P4 architecture reference, full setup walkthrough, common gotchas table, and VS Code integration notes.

## Commit Convention

Use `Co-Contributed-By: Claude Opus 4.6 <noreply@anthropic.com>` in commits.
