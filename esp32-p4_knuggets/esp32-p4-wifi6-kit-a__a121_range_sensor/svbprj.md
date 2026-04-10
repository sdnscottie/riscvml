# esp32-p4-wifi6-kit-a__a121_range_sensor

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** Acconeer A121 60GHz radar range sensor
**Status:** TODO — pinout confirmed, wiring diagram in progress
**Anki-Ref:** TBD

## Overview

SvBlock: A121 pulsed coherent radar sensor for range/distance measurement.
60GHz mmWave radar — measures distance, detects presence, motion.

## Project Folder Structure

```
esp32-p4-wifi6-kit-a__a121_range_sensor/
├── svbprj.md
├── anki_refs/
├── pics/
├── docs_about__esp32-p4-wifi6-kit-a__a121_range_sensor/
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
