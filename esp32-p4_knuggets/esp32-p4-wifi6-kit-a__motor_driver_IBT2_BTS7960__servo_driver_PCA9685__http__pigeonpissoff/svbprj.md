# esp32-p4-wifi6-kit-a__motor_driver_IBT2_BTS7960__servo_driver_PCA9685__http__pigeonpissoff

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** IBT-2 BTS7960 DC Motor + ESP32-C6 Wi-Fi
**Status:** WORKING — Pigeon.Piss-Off deployed on balcony (2026-04-06)
**Anki-Ref:** TBD

## Overview

Solution-V-Block (SVBlock): Automated pigeon deterrent.
People all over the world have a problem with pigeons pooping on their balcony.
This solution solves that!

BTS7960 DC motor with random burst patterns controlled via WiFi captive portal.
Connect from phone/tablet, configure burst timing, and let the motor scare pigeons away.

## Working Configuration

| Component | Version |
|-----------|---------|
| ESP-IDF | **v5.4.1** |
| esp_hosted | latest |
| esp_wifi_remote | latest |
| C6 slave | esp-hosted-mcu v2.12.3 |
| IDF path | `~/Dropbox/scottsoft_sdn/esp-idf-v5.4.1/` |

## Features

### Manual Motor Control
- Big toggle button with pigeon icons (OFF=poop, ON=flying away)
- Speed slider 0-100% (remembered when toggling)
- Preset buttons: 10%, 25%, 50%
- 25kHz PWM on GPIO 25, enables on GPIO 32/22

### Auto Pigeon Defense
- Random 2-5 burst ON/OFF repetitions (0.5s each)
- Configurable min/max speed (default 25%)
- Pause between bursts in minutes (slider)
- Schedule: start/end time sliders (default 08:00-16:00)
- Burst stats: count, time remaining
- Single toggle button (OFF=grey+poop, ON=green+flying)

### WiFi Captive Portal
- AP SSID: riscvml.org__pigeon_piss_off
- Password: !!scottie!!
- WPA2-PSK, auto-channel
- DNS hijack for captive portal auto-redirect
- NVS credential storage + Factory Reset
- Connect to home WiFi (AP+STA mode)

### Multi-Language Support (7 languages)
- English, Deutsch, Italiano, Espanol, Portugues, Turkce, Hindi
- Flag icons on language buttons
- Selected language highlighted neon green
- Persistent via localStorage
- All UI text translatable via data-t attributes

### UI Design
- Dark theme (#1a1a2e background)
- Neon green (#00ff88) accents
- 5 tabs: Manual, Auto, WiFi, About, Language (globe icon)
- Active tab: neon green background
- Pigeon emoji icons throughout
- Title: Pigeon.Piss-Off with pigeon emojis
- About: SVB branding + riscvml.org link

## Motor Control (BTS7960 IBT-2)

- **RPWM:** GPIO 25 (PWM speed, 25 kHz, 10-bit)
- **R_EN:** GPIO 32 (enable)
- **L_EN:** GPIO 22 (enable)
- **Direction:** Forward only (LPWM not connected)

## API Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| GET | / | Tabbed captive portal |
| GET | /api/status | JSON: sta_ip, speed, on, auto_on, bursts, time_left |
| POST | /api/motor | Motor: speed, on, off |
| POST | /api/auto | Start auto defense with config |
| POST | /api/auto/stop | Stop auto defense |
| POST | /api/wifi | Connect to home WiFi |
| POST | /api/reset | Factory reset |

## Project Folder Structure

```
rs_riscvml__...__pigeonpissoff/
├── svbprj.md
├── anki_refs/
├── pics/
├── docs_about__.../
│   └── archive__docs/          <- versioned docs
├── ...__free/esp_idf_ws/main/
└── ...__full/
    ├── archive__code/          <- versioned code (timestamped)
    │   ├── main__20260406_0800.c
    │   ├── main__20260406_0830.c
    │   ├── main__20260406_0900.c
    │   ├── main__20260406_1000.c
    │   ├── main__20260406_1015.c
    │   ├── main__20260406_1045.c
    │   └── main__20260406_1130_WORKING.c
    └── firmware_phase_1/       <- CURRENT working firmware
        ├── CMakeLists.txt
        ├── sdkconfig.defaults
        └── main/
            ├── CMakeLists.txt
            ├── idf_component.yml
            └── main.c
```

## Build & Flash

```bash
source ~/Dropbox/scottsoft_sdn/esp-idf-v5.4.1/export.sh
cd rs_riscvml__...__full/firmware_phase_1
idf.py build
idf.py -p /dev/ttyACM0 flash
```

## Versioning Convention

Before updating any script or diagram, archive with timestamp:
- `main.c` -> `archive__code/main__YYYYMMDD_HHMM.c`

## Gotchas Learned

- Nested single quotes in JS translation dict inside C strings breaks everything
- JS innerHTML with emoji needs hidden divs (HTML entities render at load, JS can copy them)
- data-t attributes for translation, data-lang for language button highlighting
- localStorage for persistent language selection
- try/catch around setLang init to prevent cascade JS failures

## Prerequisite SVBlocks

| SVBlock | Status |
|---------|--------|
| Flash C6 | WORKING |
| Simple WiFi Connect | WORKING |
| WAP Streamer HTTP | WORKING |
| Motor Driver IBT2 BTS7960 | WORKING |

## Next Steps

- Field test on balcony with real pigeons
- Add servo (PCA9685) for pan/tilt aiming
- Add A121 radar for pigeon detection -> auto-trigger
- HTTPS upgrade
- Touch display integration
