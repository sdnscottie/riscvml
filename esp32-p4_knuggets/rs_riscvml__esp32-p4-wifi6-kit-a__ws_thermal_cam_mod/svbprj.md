# svbprj.md

This file provides guidance to CC (claude.ai/code) when working with code in this repository.

## Project Overview

**rs_riscvml__esp32-p4-wifi6-kit-a__ws_thermal_cam_mod** — Dual-camera object detection knugget for the Waveshare ESP32-P4 Wi-Fi 6 Kit A. Combines a MIPI CSI visual camera with the Waveshare Thermal Camera Module (MI0802) for fused visual+thermal ML object detection.

Part of the RISCVML educational curriculum (esp32-p4_knuggets).

## Board: Waveshare ESP32-P4 Wi-Fi 6 Kit A

| Feature | Detail |
|---------|--------|
| SoC | ESP32-P4 dual-core RISC-V 400 MHz HP + 40 MHz LP |
| Companion | ESP32-C6 (Wi-Fi 6 / BLE 5 / 802.15.4 Thread) via SDIO |
| Memory | 32 MB PSRAM, 16 MB Flash |
| USB UART | CH9102 → `/dev/ttyACM0` — **requires DTR=true, RTS=false** |
| Schematic | `ESP32-P4-WIFI6-datasheet.pdf` from Waveshare wiki |

### Serial Connection Gotcha (CH9102)
The CH9102 UART does NOT support RTS-based reset. To capture serial output:
```python
import serial
ser = serial.Serial('/dev/ttyACM0', 115200, timeout=1)
ser.dtr = True   # REQUIRED — chip won't talk without DTR
ser.rts = False   # Do NOT assert RTS
```
`idf.py monitor` requires a TTY (won't work from non-interactive shells).
Boot log is often missed because serial connects after boot completes — firmware prints status banner 3x with 1s delays to ensure capture.

## Two Cameras — Same Direction

| Camera | Sensor | Resolution | Interface | I2C Addr | FPS | Status |
|--------|--------|-----------|-----------|----------|-----|--------|
| MIPI CSI Camera | OV5647 (Waveshare Raspberry Pi Camera B Rev 2.0) | 800×640 | MIPI CSI 2-lane via 22-pin Pi5 connector + 15-pin adapter | 0x36 | 50 | **ON HOLD** — needs MCLK |
| Waveshare Thermal Camera Module | Meridian MI0802 | 80×62 (4,960 px) | I2C config + SPI data | 0x40 | ~4 | **WORKING** |

## GPIO Pin Assignment (Active / Verified)

### I2C Bus (Shared) — Created by sensor_init, reused by MI0802
| Signal | ESP32-P4 GPIO | Notes |
|--------|--------------|-------|
| SDA | **GPIO7** | Shared: OV5647 (0x36) + MI0802 (0x40) |
| SCL | **GPIO8** | 400 kHz |

### Waveshare Thermal Camera Module (MI0802) — SPI + Control
| Signal | ESP32-P4 GPIO | Notes |
|--------|--------------|-------|
| SPI CLK | **GPIO20** | SPI2_HOST, 8 MHz |
| SPI MOSI | **GPIO21** | |
| SPI MISO | **GPIO22** | |
| SPI CS | **GPIO23** | Active low |
| nRESET | **GPIO4** | Active-low hardware reset |
| DATA_READY | **GPIO5** | Rising edge ISR, semaphore-based |

### CSI Camera Connector (22-pin Pi5 style)
| Pin | Signal | ESP32-P4 | Notes |
|-----|--------|----------|-------|
| 1 | 3.3V | Power | |
| 2 | SDA | GPIO7 | I2C shared bus |
| 3 | SCL | GPIO8 | I2C shared bus |
| 4 | GND | | |
| 5 | CSI IO1 | **GPIO1** | Utility GPIO (PWDN or MCLK candidate) |
| 6 | CSI IO0 | **GPIO0** | Utility GPIO (MCLK or PWDN candidate) |
| 7-9 | ? | | Not confirmed |
| 10 | GND | | |
| 11-12 | ? | | Not confirmed — could be D2/D3 lanes |
| 13 | GND | | |
| 14 | CSI CLK P | Dedicated analog | MIPI clock lane |
| 15 | CSI CLK N | Dedicated analog | MIPI clock lane |
| 16 | ? | | |
| 17 | CSI D1 P | Dedicated analog | MIPI data lane 1 |
| 18 | CSI D1 N | Dedicated analog | MIPI data lane 1 |
| 19 | GND | | |
| 20 | CSI D0 P | Dedicated analog | MIPI data lane 0 |
| 21 | CSI D0 N | Dedicated analog | MIPI data lane 0 |
| 22 | GND | | |

### Buzzer / Audio (Motion Alert via ES8311 Codec)
| Signal | Interface | Notes |
|--------|-----------|-------|
| Audio Out | I2S → ES8311 DAC → SPK connector | Onboard codec, no external GPIO needed |
| Volume | `cfg_audio_volume` (0-100) | Tunable from frontend, default 80 |

## MI0802 Driver Component (WORKING)

Custom ESP-IDF component at `esp_idf_ws/components/mi0802/`:
- `mi0802.h` — Public API, register map, frame header struct
- `mi0802.c` — I2C config, SPI frame read, GPIO ISR for DATA_READY
- Supports shared I2C bus (pass `i2c_bus` handle) or standalone bus creation
- Frame format: 80-word header + 4960 pixels (uint16_t), big-endian SPI → byte-swapped
- Temperature units: header min/max in 0.1 K, pixels in raw ADC (0.1 K)
- Celsius conversion: `(raw / 10.0) - 273.15`

### Verified Thermal Output
- Outdoor night temps: 3-9°C ambient, 20-30°C warm objects
- ~4 FPS continuous mode, 1.16s per frame
- Streams base64-encoded pixel data as JSON over serial

## CSI Camera (OV5647) — ON HOLD

### Problem
OV5647 detected on I2C (Camera=OK) but produces zero MIPI frames.

### Root Cause Analysis
The OV5647 Raspberry Pi Camera (B) Rev 2.0 has **NO onboard oscillator** — it requires an external MCLK (24-25 MHz) from the host. On a Raspberry Pi, the VideoCore GPU provides 25 MHz MCLK via the CSI connector.

The Waveshare ESP32-P4-WIFI6 board has a **22-pin Pi5-style CSI connector**. The Pi5 connector has **NO dedicated MCLK pin** — it's designed for cameras with onboard oscillators (like the SC2336, which is the Waveshare demo default).

The board has GPIO0 (CSI IO0) and GPIO1 (CSI IO1) on the connector, but the 15-pin to 22-pin adapter cable may not route these to the camera's MCLK input (15-pin pin 12).

### What Was Tried
1. PWDN GPIO scanning (drove GPIOs 2,33,52,51,26,27 LOW) — no effect
2. ESP Clock Router 24MHz MCLK on GPIO0, PWDN on GPIO1 — no effect
3. Swapped: MCLK on GPIO1, PWDN on GPIO0 — no effect
4. OV5647 register diagnostic dump added (reads 0x0100 streaming, PLL, MIPI ctrl)

### Possible Solutions
- Get SC2336 camera module (has onboard oscillator, designed for this board)
- External 24MHz oscillator wired directly to OV5647 XVCLK pin
- Verify adapter cable actually routes CSI IO0/IO1 to camera MCLK (multimeter test)
- Espressif Function EV Board camera sub-board has onboard 24MHz oscillator (Y1)

### Key Reference
- Waveshare demo: `ESP32-P4-NANO_Demo.zip` → `ESP-IDF/11_simple_video_server/`
- Default camera: SC2336 (not OV5647) with `CONFIG_CAMERA_XCLK_USE_ESP_CLOCK_ROUTER=y`
- Board profile: `CONFIG_EXAMPLE_SELECT_ESP32P4_FUNCTION_EV_BOARD_V1_5=y`
- OV5647 driver: `OV5647_ENABLE_OUT_CLOCK` macro is a no-op (line 21 of ov5647.c)
- sensor_init passes `xclk_pin = -1, pwdn_pin = -1, reset_pin = -1`

## Firmware Architecture (esp_idf_ws/main/main.c)

### Init Order
1. LDO init (MIPI PHY, channel 3, 2500 mV)
2. PSRAM frame buffer allocation (800×640×2 bytes)
3. MCLK + PWDN GPIO setup (for CSI camera, currently ineffective)
4. `example_sensor_init()` — creates I2C bus on GPIO7/8, probes OV5647
5. I2C bus scan (prints all devices found)
6. CSI pipeline setup (CSI controller → ISP RAW8→RGB565 → callbacks)
7. MI0802 init (reuses sensor_init's I2C bus handle, creates SPI2)
8. Status banner (3x with 1s delays for serial capture)
9. Launch tasks: `thermal_task` (core 0) + `camera_task` (core 1)

### Serial JSON Protocol
```json
{"type":"thermal","width":80,"height":62,"min_t":2761,"max_t":2985,"data":"<base64>","frame":42}
{"type":"camera","width":800,"height":640,"jpeg":"<base64>","frame":1}
```
- Thermal `data`: base64-encoded uint16_t[4960] (0.1 K units)
- Thermal `min_t`/`max_t`: header values in 0.1 K (divide by 10, subtract 273.15 for °C)
- Camera `jpeg`: base64-encoded JPEG (HW JPEG encoder, quality 40)

### Motion Detection (Thermal)
- **Temporal frame averaging**: accumulates `AVG_FRAMES=4` raw frames, processes average (~1 averaged frame/sec at 4 FPS)
- **Noise floor**: ~32-34 max_diff with averaging (was ~60-80 without)
- **Dead pixel detection**: IQR-based outlier mask in firmware (after warmup), neighbor-averaging in JS renderer
- **Baseline**: EMA with `cfg_baseline_alpha=0.15`, updated from averaged frames, skips dead pixels
- **Motion trigger**: `max_diff > cfg_buzz_threshold (60)` AND `cfg_buzzer_enabled=1` AND `buzz_armed=true`
- **Arm delay**: `cfg_arm_delay_ms=15000` (15s after boot for baseline stabilization)
- **Cooldown**: `cfg_cooldown_ms=5000` between buzzer triggers
- **Buzzer**: ES8311 codec via I2S → SPK quick connector, 3×100ms 1kHz beeps
- **Buzzer default**: OFF (`cfg_buzzer_enabled=0`), must be enabled from frontend Calibrate page
- All motion/buzzer params tunable via `SET key=value` serial commands from frontend
- Settings changes print `==SETTING VALUE !!` banners to serial for feedback

### Frontend SPA Features
- **Thermal heatmap**: canvas rendering with dead pixel correction, configurable clip/gamma
- **Calibration page**: rich card UI with sliders, ranges, recommended values, expandable "How it works"
- **Serial console**: auto-scroll toggle, log level filters (ERR/WRN/INF/DBG), timestamps, download
- **Motion detection settings**: firmware params sent via POST /api/set → serial SET commands
- **Image calibration**: client-side params (clip, gamma) apply instantly via blue "Apply" button

## Repositories

| Repo | Visibility | URL |
|------|-----------|-----|
| **riscvml** (monorepo) | Public | https://github.com/sdnscottie/riscvml |
| **__free** (student exercise) | Public | https://github.com/sdnscottie/rs_riscvml__esp32-p4-wifi6-kit-a__ws_thermal_cam_mod__free |
| **__full** (reference solution) | Private | https://github.com/sdnscottie/rs_riscvml__esp32-p4-wifi6-kit-a__ws_thermal_cam_mod__full |

## Build Commands

```bash
# ESP-IDF firmware (in esp_idf_ws/ directory)
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh
idf.py build
idf.py -p /dev/ttyACM0 flash
# Serial capture (CH9102 needs DTR=true, RTS=false — idf.py monitor won't catch boot)

# Rust SPA Dual-Camera Viewer (in __full/ directory)
PKG_CONFIG_PATH=~/.local/lib/pkgconfig RUSTFLAGS="-L /home/maxx/.local/lib" cargo build
cargo run -- --serial /dev/ttyACM0 --baud 115200 --port 3050
```

## Conventions

- Commits use: `Co-Contributed-By: CC Opus 4.6 <noreply@anthropic.com>`
- Follow RISCVML patterns: SQLite for telemetry, embedded-hal traits for hardware abstraction
- Diagrams go in `docs_about__esp32-p4-wifi6-kit-a__ws_thermal_cam_mod__[free|full]/`
- Regenerate diagram PNGs: `drawio --export --format png --scale 2 --output X.png X.drawio`
