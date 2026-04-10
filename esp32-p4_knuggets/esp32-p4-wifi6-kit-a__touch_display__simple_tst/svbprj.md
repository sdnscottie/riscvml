# esp32-p4-wifi6-kit-a__touch_display__simple_tst

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Display:** 10.1" DSI Capacitive Touch (800x1280, IPS, GT9271, JD9365)
**Status:** TODO — simple display + touch test
**Anki-Ref:** TBD

## Overview

SvBlock: Simple test to verify the 10.1" DSI touch display works.
Shows "Welcome to RISCVML" with a touch button counter.

This is the prerequisite test before integrating the display
with the captive portal or other SvBlocks.

## Hardware

- **Display:** Waveshare 10.1-DSI-TOUCH-A
- **Resolution:** 800x1280 IPS
- **Interface:** MIPI-DSI 2-lane
- **LCD Driver IC:** JD9365
- **Touch IC:** GT9271 (Goodix, GT911-compatible)
- **Touch I2C:** SDA=GPIO7, SCL=GPIO8
- **Backlight:** I2C control (addr 0x45, reg 0x86)
- **Connection:** FPC ribbon cable to P4 DSI connector

## Build & Flash

```bash
source ~/Dropbox/scottsoft_sdn/esp-idf-v5.4.1/export.sh
cd rs_riscvml__...__full/firmware_phase_1
idf.py build
idf.py -p /dev/ttyACM0 flash
```
