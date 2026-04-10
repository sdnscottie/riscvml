# esp32-p4-wifi6-kit-a__riscvml_wap_streamer__https

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** ESP32-C6 companion (Wi-Fi 6 via SDIO)
**Status:** TODO — next SvBlock after HTTP captive portal
**Anki-Ref:** TBD

## Overview

SvBlock: HTTPS upgrade for the WiFi AP Captive Portal.
Builds on the HTTP captive portal SvBlock by adding TLS with a self-signed certificate.

**Prerequisite SvBlocks:**
- `esp32-p4-wifi6-kit-a__flash_C6` (C6 esp_hosted slave)
- `esp32-p4-wifi6-kit-a__riscvml_wap_streamer__http` (HTTP captive portal)

**Required Stack:**
- ESP-IDF v5.4.1
- esp_hosted latest
- esp_wifi_remote latest
- esp_https_server component

## What This SvBlock Adds

- Self-signed TLS certificate (generated at build time or embedded)
- `esp_https_server` instead of `esp_http_server`
- Same captive portal UI, same API endpoints, just encrypted
- Browser will show certificate warning (expected for self-signed)

## Project Folder Structure

```
esp32-p4-wifi6-kit-a__riscvml_wap_streamer__https/
├── svbprj.md
├── anki_refs/
├── pics/
├── docs_about__esp32-p4-wifi6-kit-a__riscvml_wap_streamer__https/
├── ...__free/esp_idf_ws/main/
└── ...__full/firmware_phase_1/main/
```
