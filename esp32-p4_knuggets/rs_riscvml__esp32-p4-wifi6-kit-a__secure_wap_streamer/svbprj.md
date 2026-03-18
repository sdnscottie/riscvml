# svbprj.md

This file provides guidance to CC (claude.ai/code) when working with code in this repository.

## Project Overview

**rs_riscvml__esp32-p4-wifi6-kit-a__secure_wap_streamer** — Secure wireless AP camera streamer knugget for the Waveshare ESP32-P4 Wi-Fi 6 Kit A. The ESP32-P4 acts as a Wi-Fi access point, serving an HTTPS login page and streaming MI0802 thermal camera images to a single authenticated client.

Part of the RISCVML educational curriculum (esp32-p4_knuggets).

## Concept

A self-contained, portable thermal camera AP:
1. ESP32-P4 boots as a **Wi-Fi soft-AP** (via ESP32-C6 companion over SDIO)
2. Client connects to the AP SSID
3. Browser navigates to `https://192.168.4.1`
4. **HTTPS login screen** with username/password authentication
5. After login: live thermal camera stream (MI0802 80×62 LWIR heatmap)
6. Single-client or light-duty — designed for field/portable use

## Board: Waveshare ESP32-P4 Wi-Fi 6 Kit A

| Feature | Detail |
|---------|--------|
| SoC | ESP32-P4 dual-core RISC-V 400 MHz HP + 40 MHz LP |
| Companion | ESP32-C6 (Wi-Fi 6 / BLE 5 / 802.15.4 Thread) via SDIO |
| Memory | 32 MB PSRAM, 16 MB Flash |
| USB UART | CH9102 → `/dev/ttyACM0` — **requires DTR=true, RTS=false** |

### Serial Connection Gotcha (CH9102)
The CH9102 UART does NOT support RTS-based reset. Boot log is often missed — firmware prints status banner 3x with 1s delays.

## GPIO Pin Assignment

### I2C Bus (MI0802 config)
| Signal | ESP32-P4 GPIO | Notes |
|--------|--------------|-------|
| SDA | **GPIO7** | MI0802 I2C addr 0x40 |
| SCL | **GPIO8** | 400 kHz |

### MI0802 Thermal Camera — SPI + Control
| Signal | ESP32-P4 GPIO | Notes |
|--------|--------------|-------|
| SPI CLK | **GPIO20** | SPI2_HOST, 8 MHz |
| SPI MOSI | **GPIO21** | |
| SPI MISO | **GPIO22** | |
| SPI CS | **GPIO23** | Active low |
| nRESET | **GPIO4** | Active-low hardware reset |
| DATA_READY | **GPIO5** | Rising edge ISR |

## Architecture

```
┌─────────────────────────────────────────────┐
│           ESP32-P4 (main SoC)               │
│                                             │
│  MI0802 ──SPI──► thermal_task               │
│                    │                        │
│                    ▼                        │
│             frame buffer                    │
│                    │                        │
│                    ▼                        │
│  ESP-IDF HTTPS Server ◄── TLS (self-signed)│
│    ├─ GET /          → login.html           │
│    ├─ POST /login    → session auth         │
│    ├─ GET /stream    → thermal heatmap page │
│    └─ GET /api/frame → JSON thermal data    │
│                                             │
│  ESP32-C6 ◄──SDIO──► Wi-Fi 6 soft-AP       │
│                       SSID: RISCVML-Cam     │
│                       192.168.4.1           │
└─────────────────────────────────────────────┘
```

### Key Components
- **Wi-Fi AP**: ESP32-C6 companion chip provides Wi-Fi 6, configured as soft-AP via SDIO
- **HTTPS Server**: ESP-IDF `esp_https_server` component with self-signed TLS cert
- **Authentication**: Simple username/password login, session cookie
- **Thermal Stream**: MI0802 frames rendered as heatmap in browser (embedded HTML/JS/CSS)
- **MI0802 Driver**: Reuse component from CritterCam project (`esp_idf_ws/components/mi0802/`)

## Security Design
- Self-signed TLS certificate (generated at build time or embedded in flash)
- Login page served over HTTPS — no plaintext credentials
- Session token via HTTP cookie after successful auth
- Default credentials configurable via `sdkconfig` or menuconfig
- Single-client design limits attack surface

## Repositories

| Repo | Visibility | URL |
|------|-----------|-----|
| **riscvml** (monorepo) | Public | https://github.com/sdnscottie/riscvml |
| **__free** (student exercise) | Public | TBD |
| **__full** (reference solution) | Private | TBD |

## Build Commands

```bash
# ESP-IDF firmware (in esp_idf_ws/ directory)
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh
idf.py build
idf.py -p /dev/ttyACM0 flash monitor   # Ctrl+] to exit

# Generate self-signed TLS cert (once)
openssl req -x509 -newkey rsa:2048 -keyout server_key.pem -out server_cert.pem -days 3650 -nodes -subj '/CN=RISCVML-Cam'
```

## Conventions

- Commits use: `Co-Contributed-By: CC Opus 4.6 <noreply@anthropic.com>`
- Follow RISCVML patterns: ESP-IDF C firmware, embedded HTML/JS for frontend
- MI0802 component reused from CritterCam knugget
- Diagrams go in `docs_about__esp32-p4-wifi6-kit-a__secure_wap_streamer__full/`
