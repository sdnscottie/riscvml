# esp32-p4-wifi6-kit-a__riscvml_wap_streamer__http

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** ESP32-C6 companion (Wi-Fi 6 via SDIO)
**Status:** WORKING -- AP + STA + Captive Portal + NVS + DNS hijack (2026-04-05)
**Anki-Ref:** TBD

## Overview

SvBlock (Scrabble-v-Block): Wi-Fi Access Point with HTTP Captive Portal on the ESP32-P4.
Other SvBlocks (motor driver, touch display, servo, thermal camera) stack on top.

**Next SvBlock:** `esp32-p4-wifi6-kit-a__riscvml_wap_streamer__https`

**Prerequisite SvBlocks:**
- `esp32-p4-wifi6-kit-a__flash_C6` (C6 esp_hosted slave)
- Simple WiFi Connect SvBlock (verified STA works on IDF v5.4.1)

## Working Configuration

| Component | Version | Notes |
|-----------|---------|-------|
| ESP-IDF | **v5.4.1** | v5.3.x BROKEN (netif crash), v6.0-dev BROKEN (SDIO bug) |
| esp_hosted | **latest (*)** | Not pinned — latest works |
| esp_wifi_remote | **latest (*)** | Not pinned — latest works |
| C6 slave firmware | **esp-hosted-mcu v2.12.3** | Built from `esp-hosted-mcu/slave/` |
| ESP-IDF path | `~/Dropbox/scottsoft_sdn/esp-idf-v5.4.1/` | |

### Standard Init Order (IDF v5.4.1 -- no workarounds!)

```
1. nvs_flash_init()
2. esp_netif_init()
3. esp_event_loop_create_default()
4. esp_netif_create_default_wifi_ap()    -- BEFORE wifi_init
5. esp_netif_create_default_wifi_sta()   -- BEFORE wifi_init
6. esp_wifi_init()                       -- connects to C6 via SDIO
7. Register event handlers
8. esp_wifi_set_mode(AP or APSTA)
9. esp_wifi_set_config(AP + STA)
10. esp_wifi_start()                     -- STA_START triggers connect
```

No delays, no post-start netif creation, no workarounds. Standard ESP-IDF order.

## How It Works

1. P4 boots, creates AP + STA netifs, starts WiFi
2. AP broadcasts `riscvml.org__secure_wap` (pass: !!scottie!!, WPA2)
3. If saved credentials in NVS: auto-connects to home WiFi (AP+STA mode)
4. If no credentials: AP-only mode with captive portal
5. Connect tablet to AP, browse to http://192.168.4.1/ -- "Welcome to RISCVML"
6. Enter home SSID + password -- saved to NVS, C6 joins home network
7. Portal accessible at both http://192.168.4.1/ (AP) and http://home-ip/ (LAN)
8. Factory Reset button: clears NVS, reboots to AP-only
9. DNS hijack: all DNS queries resolve to 192.168.4.1 for captive portal auto-redirect
10. 2-minute STA timeout: if STA can't connect, falls back to AP-only

## AP Configuration

- **SSID:** riscvml.org__secure_wap
- **Password:** !!scottie!!
- **Auth:** WPA2-PSK
- **AP IP:** 192.168.4.1
- **Channel:** auto (0) -- follows STA channel in AP+STA mode
- **Max clients:** 4

## Features

- Testing/Production build modes (IS_TESTING / IS_PRODUCTION)
- Default test credentials pre-filled in form fields
- Password eye icon toggle (SVG)
- DNS hijack server on port 53 for captive portal auto-redirect
- Client MAC + IP logging in serial console
- STA disconnect reason codes in debug output
- Heartbeat with heap stats every 30s

## SvBlock Integration

| SvBlock | What It Adds |
|---------|-------------|
| riscvml_wap_streamer__https | HTTPS upgrade with self-signed cert |
| motor_driver_IBT2_BTS7960 | DC motor PWM control via /api/motor |
| servo_driver_PCA9685 | Pan/tilt servo control via /api/servo |
| touch_display | On-screen controls + status |
| thermal_camera | Live thermal heatmap stream |

Integration happens in `riscvml_projects/` where SvBlocks are combined.

## API Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET | / | Captive portal HTML |
| GET | /api/status | JSON: sta_ip, sta_connected, saved_ssid, ap_ssid |
| POST | /api/wifi | Connect to home WiFi: {"ssid":"...","pass":"..."} |
| POST | /api/reset | Factory reset: clear NVS, reboot to AP-only |
| GET | /* | Captive portal redirect (302 to /) |

## Project Folder Structure

```
esp32-p4-wifi6-kit-a__riscvml_wap_streamer__http/
├── svbprj.md                          <- this file
├── anki_refs/
├── pics/
├── docs_about__esp32-p4-wifi6-kit-a__riscvml_wap_streamer__http/
├── ...__free/esp_idf_ws/main/         <- student scaffolding
└── ...__full/
    └── firmware_phase_1/              <- Wi-Fi AP + HTTP Captive Portal
        ├── CMakeLists.txt
        ├── sdkconfig.defaults
        └── main/
            ├── CMakeLists.txt
            ├── idf_component.yml      <- esp_hosted + esp_wifi_remote (latest)
            ├── main.c                 <- captive portal firmware
            └── sta_test.c             <- minimal STA connect test
```

## Build & Flash

```bash
source ~/Dropbox/scottsoft_sdn/esp-idf-v5.4.1/export.sh
cd rs_riscvml__...__full/firmware_phase_1
idf.py build
idf.py -p /dev/ttyACM0 flash
```

## NVS Credential Storage

- Namespace: `wifi_cfg`
- Keys: `ssid`, `pass`
- Saved on successful /api/wifi POST
- Loaded on boot -- if found, starts AP+STA mode
- Cleared by /api/reset POST or `esptool.py erase_region 0x9000 0x6000`

## Troubleshooting

| Problem | Cause | Fix |
|---------|-------|-----|
| `sdmmc_card_init failed` | C6 not running esp_hosted | Flash C6 with esp-hosted-mcu v2.12.3 |
| `netif already added` crash | IDF v5.3.x bug | **Use IDF v5.4.1** |
| `dhcp client start failed` | IDF v5.3.x netif not linked | **Use IDF v5.4.1** |
| SDIO controller bug | IDF v6.0-dev bug (#17889) | **Use IDF v5.4.1** |
| AUTH_EXPIRE (reason 2) | Normal -- C6 retries 2-3x | Wait ~20s, auto-reconnects |
| STA reason 209 disconnect | Transient -- auto-recovers | No action needed |
| `httpd: error in accept (23)` | Too many DNS hijack connections | Non-critical, ignore |
| FTDI holding C6 in bad state | FTDI TX/RX lines interfere | Disconnect ALL FTDI wires |
| P4 GPIO 54 blocks C6 flash | WiFi firmware resets C6 | Flash hello_world to P4 first |
