# esp32-p4-wifi6-kit-a__flash_C6

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** ESP32-C6 companion chip (Wi-Fi 6 + BLE 5 + Thread/Zigbee)
**Status:** Soldered header to C6 (IO9, GND, RXD, TXD) | FTDI wiring planned | esp_hosted flash TODO
**Anki-Ref:** TBD

## Project Folder Structure

```
esp32-p4-wifi6-kit-a__flash_C6/
├── svbprj.md                          <- this file
├── anki_refs/                         <- Anki flashcard decks (.apkg exports)
├── pics/                              <- photos (soldered header, FTDI wiring)
├── docs_about__esp32-p4-wifi6-kit-a__flash_C6/
│   └── flash-c6-via-ftdi.drawio       <- FTDI wiring + flash process diagram
├── ...__free/                         <- student scaffolding (public)
│   └── esp_idf_ws/main/              <- ESP-IDF workspace stub
└── ...__full/                         <- reference solution (private)
    └── firmware_phase_1/main/         <- esp_hosted slave firmware build
```

## Overview

The Waveshare ESP32-P4-WIFI6 Kit A has an ESP32-C6 companion chip onboard that
provides Wi-Fi 6, BLE 5, and Thread/Zigbee via SDIO to the ESP32-P4. The C6 ships
with no firmware (or AT firmware) and must be flashed with **esp_hosted slave firmware**
before the P4 can use wireless connectivity.

The C6 has no USB connector on the Waveshare board -- only solder pads (V, D-, D+, G).
This project documents how to flash the C6 using an external FTDI USB-to-serial adapter
connected to a soldered header on the C6's UART0 pins.

**This is a prerequisite for all P4 projects that need Wi-Fi or BLE.**

## Hardware

### Soldered Header on C6

A 4-pin header has been soldered to the C6 UART0 pads on the Waveshare board:

| Pad | Function |
|-----|----------|
| IO9 | Boot mode select (LOW = download, HIGH = normal run) |
| GND | Ground |
| RXD | UART0 receive (connect to FTDI TX) |
| TXD | UART0 transmit (connect to FTDI RX) |

### FTDI Adapter

- USB-to-serial adapter with its own USB-C plug
- **Must be set to 3.3V TTL** -- 5V will damage the C6!
- Do NOT connect FTDI VCC/3V3 to the board -- C6 is already powered by the P4 board

### Two USB-C Cables to Linux PC

| USB-C Cable | Connects To | Linux Port | Purpose |
|-------------|-------------|------------|---------|
| Cable 1 | P4 board main USB-C | /dev/ttyACM0 | Flash/monitor P4 firmware |
| Cable 2 | FTDI adapter USB-C | /dev/ttyUSB0 | Flash C6 esp_hosted slave |

### Wiring: FTDI to C6 Header

```
FTDI Adapter          C6 Soldered Header
────────────          ──────────────────
TX          ───────→  RXD
RX          ←───────  TXD
GND         ───────→  GND
(no VCC!)             IO9 → GND (jumper for boot mode)
```

## Download Mode

ESP32-C6 boot mode is controlled by IO9 at reset:

| IO9 State | Boot Mode |
|-----------|-----------|
| LOW (GND) | UART download mode -- ready for flashing |
| HIGH (floating/pulled up) | Normal run mode -- executes firmware |

### Flash Sequence

1. Jumper IO9 → GND on C6 header
2. Reset C6 (power cycle board, or P4 toggles GPIO 54 = C6 reset line)
3. Verify FTDI detected: `ls /dev/ttyUSB0`
4. Flash via FTDI: `idf.py -p /dev/ttyUSB0 flash`
5. Remove IO9 jumper
6. Reset C6 → boots esp_hosted slave firmware
7. P4 now has Wi-Fi via SDIO bus (GPIO 14-19)

## Building esp_hosted Slave Firmware

```bash
# Activate ESP-IDF
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh

# Clone esp_hosted
git clone --recursive https://github.com/espressif/esp-hosted.git
cd esp-hosted/esp_hosted_fg/esp/esp_driver/network_adapter

# Build for C6 with SDIO transport
idf.py set-target esp32c6
idf.py menuconfig    # → Transport: SDIO
idf.py build

# Flash via FTDI (with IO9 jumpered to GND)
idf.py -p /dev/ttyUSB0 flash
```

## GPIO References

### C6 ↔ P4 SDIO Bus (DO NOT USE these P4 GPIOs)

| P4 GPIO | Function |
|---------|----------|
| 14, 15, 16, 17 | SDIO D0-D3 |
| 18 | SDIO CLK |
| 19 | SDIO CMD |
| 54 | C6 Reset line |

## Diagrams

- `flash-c6-via-ftdi.drawio` -- full wiring diagram showing both USB-C connections, FTDI-to-C6 jumper wires, SDIO bus, and flash process steps

Regenerate PNG: `drawio --export --format png --scale 2 --output flash-c6-via-ftdi.png flash-c6-via-ftdi.drawio`

## This Unblocks

- Motor driver Wi-Fi STA (esp32-p4-wifi6-kit-a__motor_driver_IBT2_BTS7960, Phase 3)
- Secure WAP Streamer Wi-Fi AP
- CritterCam network streaming
- Any P4 project needing Wi-Fi or BLE
