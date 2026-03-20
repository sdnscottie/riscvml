# rs_riscvml__esp32-p4-wifi6-kit-a__multimeter

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** DIY Digital Multimeter — Voltage Measurement
**Status:** Planning

## Overview

A DIY digital multimeter built on the ESP32-P4, using the onboard ADC to measure voltage
through a resistor divider, displayed on a MIPI-DSI LCD or SPI TFT screen.

This knugget teaches:
- ADC (Analog-to-Digital Converter) fundamentals
- Resistor voltage divider design
- Calibration and accuracy
- Real-time display rendering (LVGL or direct framebuffer)
- Input protection circuits
- Wi-Fi data logging (voltage readings to SQLite)

## How It Works

```
Probe Tips (+/−)
      │
      ▼
┌─────────────┐
│  Resistor   │     Divides input voltage down to 0–3.3V range
│  Divider    │     for safe ADC reading
│  + Clamp    │     Zener/TVS diode protects ADC input
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  ESP32-P4   │     ADC reads 0–3.3V → 12-bit (0–4095)
│  ADC Input  │     Firmware converts to actual voltage
│  (GPIO TBD) │     using divider ratio + calibration
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Display    │     Shows voltage reading in real-time
│  (LCD/TFT)  │     Big digits, bar graph, min/max/avg
└─────────────┘
```

## Measurement Ranges

| Range | Divider Ratio | Max Input | ADC Sees | Resolution |
|-------|--------------|-----------|----------|------------|
| 0–3.3V | 1:1 (direct) | 3.3V | 0–3.3V | ~0.8mV |
| 0–12V | 1:4 (3k+1k) | 12V | 0–3.0V | ~3mV |
| 0–30V | 1:10 (9k+1k) | 30V | 0–3.0V | ~7.3mV |
| 0–50V | 1:17 (16k+1k) | 50V | 0–2.94V | ~12mV |

Default range for this project: **0–30V** (covers 12V and 24V systems, drill batteries up to 20V).

## Voltage Divider Circuit

```
Probe +  ────┬──── R1 (9kΩ) ────┬──── R2 (1kΩ) ────┬──── Probe −
             │                   │                    │
             │                   │                    │
             │              ADC Input              GND (ESP32-P4)
             │              (GPIO TBD)
             │                   │
             │              ┌────┴────┐
             │              │ 3.3V    │  Zener/TVS clamp
             │              │ Zener   │  protects ADC if
             │              └────┬────┘  input > 30V
             │                   │
             │                  GND
```

**Formula:** `V_input = V_adc × (R1 + R2) / R2 = V_adc × 10`

## Components

| Component | Value | Purpose | Price |
|-----------|-------|---------|-------|
| R1 | 9kΩ (precision 1%) | Upper divider resistor | ~€0.10 |
| R2 | 1kΩ (precision 1%) | Lower divider resistor | ~€0.10 |
| D1 | 3.3V Zener (BZX55C3V3) | ADC input protection | ~€0.10 |
| C1 | 100nF ceramic | ADC input smoothing | ~€0.05 |
| Banana jacks | 2x (red+black) | Probe connection points | ~€2.00 |
| Test probes | Pair with banana plugs | Measurement probes | ~€3.00 |
| Display | SPI TFT 1.8" ST7735 or MIPI-DSI | Voltage readout | ~€3–15 |
| **Total BOM** | | | **~€8–20** |

## Display Layout

```
┌──────────────────────────────┐
│  RISCVML Multimeter    DC V  │
│                              │
│      ██  ██  ██  ██          │
│      ██  ██. ██  ██  V       │
│      ██  ██  ██  ██          │
│                              │
│  ▓▓▓▓▓▓▓▓▓▓▓░░░░░░░  65%   │
│  0V            15V     30V   │
│                              │
│  MIN: 11.8V  MAX: 12.4V     │
│  AVG: 12.1V  Samples: 1024  │
│                              │
│  ⚡ Wi-Fi: connected         │
│  📊 Logging to SQLite        │
└──────────────────────────────┘
```

## GPIO Pin Assignment (Waveshare ESP32-P4-WIFI6 Kit A)

**Note:** Only GPIOs 22, 25, 32 confirmed working on 40-pin header (bench tested 2026-03-20).

| GPIO | Function | Side |
|------|----------|------|
| 25 | ADC input (voltage divider output) | Left |
| 22 | Display SPI CS (or I2C SDA) | Right |
| 32 | Display SPI DC (or I2C SCL) | Right |

> **TODO:** Need to test if GPIO 25 supports ADC on ESP32-P4, and identify more working GPIOs
> for SPI display (needs CS, DC, CLK, MOSI = 4 pins). May use MIPI-DSI display instead.

## Firmware Features

### Phase 1 — Basic Voltage Reading
- ADC read on GPIO (12-bit, 0–4095)
- Voltage divider math: `V = adc_raw × (3.3 / 4095) × divider_ratio`
- Serial output: `printf("Voltage: %.2fV\n", voltage)`
- 10 readings/sec, moving average filter

### Phase 2 — Display
- SPI TFT or MIPI-DSI LCD
- Big 7-segment style digits (LVGL or custom rendering)
- Bar graph showing % of range
- MIN / MAX / AVG tracking
- Auto-range detection (switch divider ratio if needed)

### Phase 3 — Wi-Fi + Logging
- Wi-Fi AP mode (like crittercam pattern)
- Web dashboard: `/` shows live voltage reading
- API: `GET /api/voltage` returns JSON `{"voltage": 12.34, "min": 11.8, "max": 12.4}`
- SQLite logging: timestamp + voltage to `multimeter.db`
- CSV export via `/api/export`

### Phase 4 — Advanced
- AC voltage measurement (via rectifier + peak detect circuit)
- Current measurement (via shunt resistor + INA219 I2C module)
- Resistance measurement (known current source + voltage reading)
- Continuity buzzer (GPIO output to piezo)
- Frequency counter (pulse counting on GPIO input)

## Safety

- **Input protection:** 3.3V Zener clamp on ADC input — prevents damage if probe touches >30V
- **Max input voltage:** 30V DC (with 1:10 divider). NEVER connect to mains voltage (120V/230V AC)
- **Isolation:** No galvanic isolation — probe GND is connected to ESP32 GND
- **Accuracy:** ±2% with 1% resistors, ±0.5% after calibration against known reference

## Calibration

1. Measure a **known voltage source** (e.g., fresh 9V battery reads 9.4V on your Fluke)
2. Read the ADC raw value at that voltage
3. Calculate correction factor: `cal_factor = known_voltage / measured_voltage`
4. Store `cal_factor` in NVS (non-volatile storage) — persists across reboots
5. Apply: `calibrated_voltage = raw_voltage × cal_factor`

## Project Structure

```
rs_riscvml__esp32-p4-wifi6-kit-a__multimeter/
├── svbprj.md                          ← this file
├── docs_about__multimeter/            ← wiring diagrams, display mockups
├── anki_refs/                         ← flashcard cross-references
├── pics/                              ← build photos
├── ...__multimeter__free/             ← student scaffolding (public)
└── ...__multimeter__full/             ← reference solution (private)
```

## Build & Flash

```bash
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh
cd rs_riscvml__esp32-p4-wifi6-kit-a__multimeter__full
idf.py build
idf.py -p /dev/ttyACM0 flash        # flash via ACM0
# Serial monitor on /dev/ttyACM1 at 115200 baud
```

## Applied to riscvml.org / agrarobotics.com

- **riscvml.org:** Curriculum project teaching ADC, voltage dividers, display rendering, Wi-Fi APIs
- **agrarobotics.com:** Field voltage monitor for solar panels, battery banks, motor driver debugging
- **Practical use:** Debug tool for other knuggets — measure B+/B- on BTS7960 without needing a Fluke
