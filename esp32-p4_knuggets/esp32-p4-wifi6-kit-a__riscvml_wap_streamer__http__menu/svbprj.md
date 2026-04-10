# esp32-p4-wifi6-kit-a__riscvml_wap_streamer__http__menu

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** ESP32-C6 companion (Wi-Fi 6 via SDIO)
**Status:** TODO
**Anki-Ref:** TBD

## Overview

SvBlock: HTTP menu system for the WAP Streamer captive portal.
Builds on the HTTP captive portal SvBlock by adding a menu/navigation structure.

**Prerequisite SvBlocks:**
- `esp32-p4-wifi6-kit-a__riscvml_wap_streamer__http` (WORKING)

**Required Stack:**
- ESP-IDF v5.4.1
- esp_hosted + esp_wifi_remote (latest)

## Project Folder Structure

```
esp32-p4-wifi6-kit-a__riscvml_wap_streamer__http__menu/
├── svbprj.md
├── anki_refs/
├── pics/
├── docs_about__esp32-p4-wifi6-kit-a__riscvml_wap_streamer__http__menu/
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
