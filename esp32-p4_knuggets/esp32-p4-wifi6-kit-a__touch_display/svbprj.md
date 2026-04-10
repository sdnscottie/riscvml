# esp32-p4-wifi6-kit-a__touch_display

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** MIPI-DSI touchscreen display
**Status:** TODO
**Anki-Ref:** TBD

## Overview

SvBlock: Touch display for the ESP32-P4 — on-device GUI control.
Uses MIPI-DSI interface with LVGL for touch UI.

When combined with the WiFi captive portal SvBlock, provides both
browser-based and on-screen control of all connected SvBlocks.

**Prerequisite SvBlocks:**
- First Steps SvBlock

**Enables:**
- On-device motor control (no phone/tablet needed)
- On-device WiFi configuration
- On-device thermal camera heatmap display
- Standalone operation without any external device

## ESP32-P4 Display Features

- MIPI-DSI interface (up to 1080p)
- Capacitive touch inputs
- PPA (Pixel Processing Accelerator) — hardware scaling, rotation
- 2D-DMA for framebuffer operations
- LVGL v8/v9 support

## Project Folder Structure

```
esp32-p4-wifi6-kit-a__touch_display/
├── svbprj.md
├── anki_refs/
├── pics/
├── docs_about__esp32-p4-wifi6-kit-a__touch_display/
├── ...__free/esp_idf_ws/main/
└── ...__full/firmware_phase_1/main/
```

## Build & Flash

```bash
source ~/Dropbox/scottsoft_sdn/esp-idf-v5.4.1/export.sh
cd rs_riscvml__...__full/firmware_phase_1
idf.py build
idf.py -p /dev/ttyACM0 flash
```
