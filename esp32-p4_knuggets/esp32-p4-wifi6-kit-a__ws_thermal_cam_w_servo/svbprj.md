# esp32-p4-wifi6-kit-a__ws_thermal_cam_w_servo

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Modules:** OV5647 CSI camera + SPT17HV-180 HV servo (pan) + Wi-Fi AP via C6 esp_hosted
**Status:** 2026-04-09 — Wi-Fi + HTTP + 1st camera frame WORKING; CSI frame loop stuck at 1 frame (next session)
**Anki-Ref:** TBD

## Overview

**Triple-integration knugget** combining three already-proven sub-projects into one unified firmware:

1. **`ws_thermal_cam_mod`** — MI0802 thermal + OV5647 CSI camera driver, ESP-IDF C firmware (copied as base)
2. **`servo_driver_direct`** — Direct LEDC servo drive on GPIO 25, verified 2026-04-09 with SPT17HV-180 on 2S 21700 pack
3. **`motor_driver_IBT2_BTS7960__servo_driver_PCA9685__http__pigeonpissoff`** — Wi-Fi AP + HTTP server pattern (esp_hosted + esp_wifi_remote)

The result is a **Wi-Fi-accessible camera node with a pan servo** — connect a phone/laptop to the ESP32-P4's own Wi-Fi, open a browser, see live camera snapshots refreshed every 5 seconds.

Ultimate target use case: **smart mouse trap surveillance** — camera watches the trap area, servo actuates the trap door on detection, all controllable over Wi-Fi.

## Current Status (2026-04-09, end of session)

### ✅ What works
- **OV5647 CSI camera detected and initialized** — I2C 0x36, PID 0x5647, 800×640 RAW8 @ 50fps
- **MCLK fix discovered:** MCLK on **GPIO 1**, PWDN on **GPIO 0** (swapped from prior "ON HOLD" memory — the cable DID route MCLK, just on the opposite pin from what the naming suggested)
- **Wi-Fi AP up** — `riscvml_cam` / `!!scottie!!` via esp_hosted-mcu v2.12.3 on C6 chip over SDIO
- **HTTP server running** on port 80
- **Phone connects successfully** — DHCP 192.168.4.2, browser loads root HTML page
- **First camera frame captured and served** via `GET /camera.jpg`
- **Servo pan task still running** on GPIO 25 (55°→100° sweep) unaffected by the rest
- **esp_hosted on IDF v6.0-dev works** — prior `feedback_idf_version_critical.md` memory claiming v5.4.1 is mandatory was wrong

### ❌ Known issues (resume point for next session)
- **Only Camera #1 is captured per boot.** `esp_cam_ctlr_receive()` blocks forever on frame 2+, even with `ESP_CAM_CTLR_MAX_DELAY`. Browser shows the boot frame but doesn't refresh.
- **MI0802 thermal cam I2C fails** with `ESP_ERR_INVALID_STATE` — shared-bus handoff issue when OV5647 owns the I2C bus. Separate investigation; not blocking OV5647.

## GPIO Pin Map

### Camera (OV5647)
| Signal | GPIO | Notes |
|--------|------|-------|
| I2C SDA | 7 | Shared bus, OV5647 at 0x36 |
| I2C SCL | 8 | 400 kHz |
| **MCLK** | **1** | **24 MHz via esp_clock_output (CORRECT pin for this cable)** |
| **PWDN** | **0** | Driven LOW to enable camera |
| MIPI CSI | 22-pin Pi5 connector | 2-lane RAW8, dedicated analog pins |

### MI0802 Thermal (inherited, I2C currently broken)
| Signal | GPIO | Notes |
|--------|------|-------|
| SPI CLK | 20 | SPI2_HOST, 8 MHz |
| SPI MOSI | 21 | |
| SPI MISO | 22 | |
| SPI CS | 23 | Active low |
| nRESET | 4 | Active-low reset |
| DATA_READY | 5 | Rising edge IRQ |

### Servo (SPT17HV-180 pan)
| Signal | GPIO | Notes |
|--------|------|-------|
| PWM | **25** | LEDC_TIMER_1 / LEDC_CHANNEL_1, 50 Hz, 13-bit |

### Reserved (do not reuse)
GPIO 7/8 (shared I2C), 14–19 (C6 SDIO), 26/27 (USB OTG), 54 (C6 reset), 0/1 (CSI MCLK/PWDN), 4/5 (MI0802 control), 20–23 (MI0802 SPI), 25 (servo)

### Power architecture
- **ESP32-P4 board:** USB from PC (always)
- **MI0802 thermal cam:** board 3.3V (on ribbon cable)
- **OV5647 CSI camera:** powered via 22-pin Pi5 connector from board
- **Servo (SPT17HV-180):** external **2S 21700 pack** (7.4V nominal, 7.6V measured) direct, no buck — common GND to P4. See `feedback_servo_power_2s_21700.md`.

## Wi-Fi Config

| Parameter | Value |
|-----------|-------|
| SSID | `riscvml_cam` |
| Password | `!!scottie!!` |
| Mode | AP-only (WIFI_MODE_AP) |
| Auth | WPA2-PSK |
| Max clients | 4 |
| IP | 192.168.4.1 |
| DHCP | Enabled (lwip default) |
| Portal URL | http://192.168.4.1/ |
| Snapshot URL | http://192.168.4.1/camera.jpg |
| Status URL | http://192.168.4.1/api/status |

**C6 chip (ESP32-P4-WIFI6 Kit A companion)** is flashed with **esp_hosted-mcu v2.12.3 slave firmware** (done 2026-04-09 — see `flash_C6` project). This is a hard prerequisite for any Wi-Fi on this board.

## Project Folder Structure

```
rs_riscvml__...__ws_thermal_cam_w_servo/
├── svbprj.md                          ← this file
├── anki_refs/
├── pics/
├── docs_about__ws_thermal_cam_w_servo/
│   └── esp32p4_to_mi0802_and_spt17hv_wiring__20260409.drawio
└── ...__full/
    ├── Cargo.toml, Cargo.lock, src/, static/  ← Rust axum SPA (from thermal_cam_mod, unused in this project)
    └── esp_idf_ws/                            ← ESP-IDF C firmware
        ├── main/
        │   ├── main.c                          ← merged: CSI + MI0802 + audio + servo + Wi-Fi + HTTP
        │   ├── CMakeLists.txt                  ← REQUIRES list extended with esp_wifi, esp_netif, esp_event, nvs_flash, esp_http_server, lwip
        │   └── idf_component.yml               ← deps: sensor_init, esp_codec_dev, esp_wifi_remote, esp_hosted
        ├── components/mi0802/                  ← MI0802 driver (inherited from thermal_cam_mod)
        ├── CMakeLists.txt
        └── sdkconfig.defaults                  ← target=esp32p4
```

## Build & Flash

```bash
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh

cd .../ws_thermal_cam_w_servo__full/esp_idf_ws
idf.py set-target esp32p4
idf.py build
idf.py -p /dev/ttyACM0 flash

# Serial monitor: rs_serialmon (CH9102 needs DTR=true, RTS=false)
# idf.py monitor won't catch the boot log with this CH9102 quirk
```

**Build size:** ~930 KB binary, 11% free in app partition.

## Main.c Architecture

### Init order (app_main)
1. Create shared I2C bus on GPIO 7/8
2. Start 24 MHz MCLK on **GPIO 1** via esp_clock_output
3. Drive PWDN (GPIO 0) LOW to enable OV5647
4. Probe I2C 0x36 → call `example_sensor_init()` → OV5647 driver registers itself
5. Configure CSI controller (`esp_cam_new_csi_ctlr`) + ISP (RAW8→RGB565)
6. Register CSI callbacks (`on_get_new_trans` / `on_trans_finished`)
7. `esp_cam_ctlr_start()`
8. Init ES8311 audio codec (I2S + DAC)
9. Init MI0802 thermal cam (I2C currently failing here — known issue)
10. Print status banner 3× with 1 s delays (so `rs_serialmon` catches it)
11. Allocate MJPEG shared buffer + mutex + signal sem
12. `cam_wifi_init()` — NVS init, netif, esp_hosted transport, Wi-Fi AP mode, start
13. `cam_start_webserver()` — HTTP server with 3 URI handlers
14. Launch tasks: `thermal_task` (core 0), `camera_task` (core 1), `servo_task`
15. Main loop: serial command parser (`cmd_check` + `cmd_process`)

### HTTP routes
| URI | Method | Handler | Returns |
|-----|--------|---------|---------|
| `/` | GET | `cam_root_handler` | HTML page with `<img>` auto-refreshing via JS every 5 s |
| `/camera.jpg` | GET | `cam_jpeg_handler` | Latest JPEG from shared buffer, `Cache-Control: no-store` |
| `/api/status` | GET | `cam_status_handler` | JSON `{"ok":true,"cam":"riscvml_cam"}` (silences Android captive-portal 404 spam) |

### CSI Frame Pattern (current — NOT yet working past frame 1)
Adapted from `esp-idf/examples/peripherals/camera/mipi_isp_dsi/main/mipi_isp_dsi_main.c`:
- `queue_items = 1`
- `on_get_new_trans` returns **false** (fills buffer from user_data)
- `on_trans_finished` returns **false** (no sem, no work)
- File-scope `s_trans_cfg` passed as user_data to `register_event_callbacks` AND as argument to `receive()` — **same pointer** (critical for pointer identity)
- `camera_task` runs a tight loop: `receive(MAX_DELAY) → JPEG encode → mutex-copy to Wi-Fi shared buffer → log count`
- Serial JSON output (`send_camera_json`) **disabled** — at 115200 baud it was starving the CSI driver (~1.2 s per frame to base64 + transmit)

## Phase Plan

| Phase | Goal | Status |
|-------|------|--------|
| Phase 0 | Hardware identified (OV5647, MI0802, SPT17HV-180) | ✅ done |
| Phase 1 | Camera + servo + Wi-Fi all init successfully | ✅ done |
| Phase 1.5 | CSI loop streams continuously (≥1 fps) | ⏳ **stuck at frame 1 — resume here** |
| Phase 2 | Browser shows live refreshing snapshots over Wi-Fi | ⏳ blocked by 1.5 |
| Phase 3 | Fix MI0802 shared-I2C bus handoff | TBD |
| Phase 4 | Hot-spot detection (thermal) → auto-aim servo | TBD |
| Phase 5 | Add trap-door servo trigger + notification | TBD |
| Integration | Full smart mouse trap product | TBD |

## Related Projects / Memory

- `project_servo_driver_direct.md` — servo subsystem verification
- `feedback_servo_power_2s_21700.md` — power topology standard
- `project_smart_mouse_trap.md` — target application
- `project_csi_camera_debug.md` — OV5647 resolution (was "ON HOLD", now working)
- `project_ws_thermal_cam_w_servo_status.md` — debugging resume point (written 2026-04-09)
- `feedback_collaboration_spirit.md` — how to work with Scottie

## Resume Plan (Next Session)

1. **Power-cycle board** with the last flash (pointer-identity fix + serial JSON disabled). Check if `Camera #N` now counts past 1.
2. **If still stuck at frame 1:** run the unmodified Espressif example `esp-idf/examples/peripherals/camera/mipi_isp_dsi/` on the same board with the OV5647 ribbon attached. Confirms CSI pipeline works at all on this hardware + IDF combo.
3. **If espressif example works standalone:** bisect the diff between that example and our merged `main.c` to find the regression trigger. Likely suspects:
   - Wi-Fi stack (esp_hosted SDIO interrupts) preempting CSI ISR
   - Audio codec (ES8311 + I2S) sharing DMA with CSI/ISP
   - MI0802 thermal init corrupting CSI state
4. **Fallback if CSI remains broken:** simplify by removing MI0802 + audio init entirely from main.c — reduce to "just OV5647 + Wi-Fi + servo" to isolate the bug.

## Open Questions

1. Does CSI DMA share any resource with esp_hosted SDIO on the ESP32-P4?
2. Is there an init-order dependency where Wi-Fi must come up before CSI (or vice versa)?
3. Does the MI0802 init's I2C bus failure leave the bus in a corrupt state that affects subsequent CSI frame delivery?
