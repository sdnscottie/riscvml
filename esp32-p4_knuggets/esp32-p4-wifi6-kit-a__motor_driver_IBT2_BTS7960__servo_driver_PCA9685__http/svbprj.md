# esp32-p4-wifi6-kit-a__motor_driver_IBT2_BTS7960__servo_driver_PCA9685__http

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** IBT-2 BTS7960 + PCA9685 + ESP32-C6 Wi-Fi
**Status:** Motor WiFi control WORKING (2026-04-06) | Servo TODO
**Anki-Ref:** TBD

## Overview

Combined SvBlock: Motor driver + Servo driver + HTTP WiFi captive portal.
Control BTS7960 DC motor via browser over WiFi. Tabbed menu interface.

Motor speed slider, ON/OFF, preset buttons — all driving real GPIOs.

## Working Configuration

| Component | Version | Notes |
|-----------|---------|-------|
| ESP-IDF | **v5.4.1** | Standard init order, no workarounds |
| esp_hosted | **latest** | Not pinned |
| esp_wifi_remote | **latest** | Not pinned |
| C6 slave | **esp-hosted-mcu v2.12.3** | |

## Prerequisite SvBlocks

| SvBlock | Status |
|---------|--------|
| Flash C6 | WORKING |
| Simple WiFi Connect | WORKING |
| WAP Streamer HTTP Menu | WORKING |
| Motor Driver IBT2 BTS7960 | **WORKING** (GPIO 25/32/22) |
| Servo Driver PCA9685 | TODO |

## Motor Control (BTS7960 IBT-2)

- **RPWM:** GPIO 25 (PWM speed, 25 kHz, 10-bit)
- **R_EN:** GPIO 32 (enable)
- **L_EN:** GPIO 22 (enable)
- **Direction:** Forward only (LPWM not connected — needs 4th GPIO for reverse)
- **API:** POST /api/motor `{"cmd":"speed","val":50}` or `{"cmd":"on"}` or `{"cmd":"off"}`

## Captive Portal Tabs

| Tab | Function | Status |
|-----|----------|--------|
| Home | System status, AP/STA info | WORKING |
| WiFi | Connect to home WiFi, Factory Reset | WORKING |
| Motor | BTS7960 speed slider, ON/OFF, presets | **WORKING** |
| Trip | PB Trip Tracker (KM, receipts) | Placeholder |
| AirBNB | Guest WiFi key code portal | Placeholder |
| Others | Future SvBlock showcase | Placeholder |

## API Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET | / | Tabbed captive portal HTML |
| GET | /api/status | JSON: sta_ip, sta_connected, saved_ssid |
| POST | /api/motor | Motor control: speed, on, off |
| POST | /api/wifi | Connect to home WiFi |
| POST | /api/reset | Factory reset (clear NVS, reboot) |
| GET | /* | Captive portal redirect |

## AP Configuration

- **SSID:** riscvml.org__secure_wap
- **Password:** !!scottie!!
- **Auth:** WPA2-PSK
- **AP IP:** 192.168.4.1

## Project Folder Structure

```
rs_riscvml__...__http/
├── svbprj.md
├── anki_refs/
├── pics/
├── docs_about__.../
│   └── archive__docs/          <- versioned docs (timestamped)
├── ...__free/esp_idf_ws/main/
└── ...__full/
    ├── archive__code/          <- versioned code (timestamped)
    │   ├── firmware_phase_1/   <- old GPIO sweep
    │   ├── firmware_phase_2/   <- old SPA web control
    │   ├── firmware_phase_2__on_off_test/
    │   ├── firmware_phase_2__tst__motor_pwm_FWD_REV/
    │   ├── firmware_phase_2__tst__motor_pwm_FWD_REV__two_speed_rnd/
    │   ├── firmware_phase_3/
    │   ├── firmware_phase_3__tst__motor_pwd_FWD_REV__rs_app_cntrl/
    │   └── firmware_wifi_captive_portal/
    └── firmware_phase_1/       <- CURRENT: tabbed portal + motor control
        ├── CMakeLists.txt
        ├── sdkconfig.defaults
        └── main/
            ├── CMakeLists.txt
            ├── idf_component.yml
            └── main.c          <- WiFi AP + Motor GPIO + HTTP server
```

## Build & Flash

```bash
source ~/Dropbox/scottsoft_sdn/esp-idf-v5.4.1/export.sh
cd rs_riscvml__...__full/firmware_phase_1
idf.py build
idf.py -p /dev/ttyACM0 flash
```

## Versioning Convention

Before updating any script or diagram, archive current version with timestamp:
- `main.c` → `archive__code/main__YYYYMMDD_HHMM.c`
- `diagram.drawio` → `archive__docs/diagram__YYYYMMDD_HHMM.drawio`

## NVS Credential Storage

- Namespace: `wifi_cfg`
- Keys: `ssid`, `pass`
- Saved on /api/wifi POST
- Cleared by /api/reset POST or `esptool.py erase_region 0x9000 0x6000`

## Next Steps

- Integrate PCA9685 servo driver (I2C Bus 1, GPIO 20/21)
- Add servo pan/tilt tab to portal
- Create pinout__motor_servo_http diagram
- Test motor + WiFi simultaneously under load
