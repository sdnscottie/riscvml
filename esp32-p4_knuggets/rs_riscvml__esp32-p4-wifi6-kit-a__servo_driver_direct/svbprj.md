# rs_riscvml__esp32-p4-wifi6-kit-a__servo_driver_direct

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** Hobby servo (SG90 / MG90S / MG996R) driven directly from ESP32-P4 GPIO via LEDC PWM
**Status:** Project created 2026-04-09 | Phase 1 (single servo sweep) TODO
**Anki-Ref:** TBD

## Overview

Puzzle-piece knugget that drives hobby servos **directly** from the ESP32-P4 using the
LEDC peripheral to generate 50 Hz PWM — no PCA9685 required. Ideal for small servo
counts (1–4) where the extra I2C breakout isn't justified.

Companion to:
- `rs_riscvml__esp32-p4-wifi6-kit-a__servo_driver_PCA9685` (I2C driver, 16-ch)
- `rs_riscvml__esp32-p4-wifi6-kit-a__motor_driver_IBT2_BTS7960` (DC motor)

### Direct Drive vs PCA9685

| Aspect | Direct (this project) | PCA9685 |
|--------|-----------------------|---------|
| Max servos | ~4 (GPIO-limited on this board) | 16 per module, daisy-chainable |
| Wiring | 1 signal per servo | 2 wires total (SDA + SCL) |
| CPU load | LEDC hardware, negligible | Offloaded to PCA9685 |
| Power | External 5–6V supply required | External 5–6V supply required |
| Failure points | Fewer (no I2C module) | One more component |
| Best for | Camera pan/tilt, 1–2 servos | Robotic arms, hexapods, >4 servos |

## Hardware

### Servo (Phase 1: single servo)

- Standard hobby servo, 50 Hz PWM, 1.0–2.0 ms pulse (extended range 0.5–2.5 ms for full 180°)
- 5–6V supply via V+ (NOT from ESP32-P4 3.3V rail)
- Signal wire = 3.3V logic (servos trigger on edges, no level shift needed)

### Power

> **DO NOT power servos from the board's 3.3V or 5V rail.** Even an SG90 can pull
> 500 mA+ stall current and brown out the ESP32-P4.

Use a separate 5–6V supply (UBEC, BEC, bench PSU, or battery pack) with **common GND**
tied to the ESP32-P4 ground.

### Wiring (Phase 1)

```
ESP32-P4 (40-Pin Header)        Servo             External 5–6V Supply
─────────────────────            ─────             ─────────────────────
GPIO ?? ────────────────────►    Signal (orange/white)
                                 V+     (red)  ◄── +5V
GND     ────────────────────►    GND    (brown/black) ◄── GND (common)
```

**All grounds must be common** (ESP32-P4 GND ↔ supply GND ↔ servo GND).

## GPIO Selection (Pending)

**Problem:** Bench test on this board (2026-03-20) found only **3 confirmed working
GPIOs on the 40-pin header: 22, 25, 32** — and all three are claimed by the IBT-2 motor
driver. Need to test additional pins from the untested pool to free up at least 1 GPIO
for the servo signal.

### Untested GPIOs (need Fluke verification)

24, 28, 29, 30, 31, 34, 36, 49, 50, 51, 52

### Reserved GPIOs (do not use)

| GPIO             | Used By                     |
|------------------|-----------------------------|
| 7, 8             | MI0802 I2C Bus 0            |
| 14–19            | SDIO D0–D3 / CLK / CMD (C6) |
| 22, 25, 32       | IBT-2 motor driver          |
| 26, 27           | USB OTG D-/D+               |
| 54               | C6 Reset                    |

## LEDC Configuration (Phase 1)

| Parameter       | Value |
|-----------------|-------|
| Timer           | LEDC_TIMER_0 |
| Speed mode      | LEDC_LOW_SPEED_MODE |
| Frequency       | 50 Hz |
| Resolution      | 13-bit (8192 steps) |
| Period          | 20 ms |
| Pulse 0°        | 0.5 ms → duty ≈ 205 |
| Pulse 90°       | 1.5 ms → duty ≈ 614 |
| Pulse 180°      | 2.5 ms → duty ≈ 1024 |

### Angle → duty formula (13-bit, 50 Hz)

```c
// pulse_us: 500..2500 → duty: 205..1024
uint32_t angle_to_duty(uint8_t angle_deg) {
    uint32_t pulse_us = 500 + (angle_deg * 2000) / 180;
    return (pulse_us * 8192) / 20000;
}
```

## Project Folder Structure

```
rs_riscvml__esp32-p4-wifi6-kit-a__servo_driver_direct/
├── svbprj.md                          ← this file
├── anki_refs/                         ← Anki flashcard decks (.apkg exports)
├── pics/                              ← photos of physical build
├── docs_about__servo_driver_direct/   ← diagrams & docs
└── ...__full/
    └── firmware_phase_1/              ← Phase 1: single servo 0→180 sweep
        └── main/
```

## Phase Plan

| Phase | Goal | Proves |
|-------|------|--------|
| Phase 1 | Single servo 0→180° sweep on LEDC | Direct PWM works, GPIO is viable |
| Phase 2 | Two servos (pan/tilt) on two LEDC channels | Multi-channel timing OK |
| Phase 3 | SPA web control (pan/tilt sliders) | HTTP API integration |
| Integration | Combine into secure_wap_streamer | End-to-end crittercam mount |

## Build & Flash

```bash
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh

cd rs_riscvml__...__full/firmware_phase_1
idf.py set-target esp32p4
idf.py build
idf.py -p /dev/ttyACM0 flash monitor
```

## Next Steps

1. Find a working GPIO (Fluke sweep untested pins)
2. Wire up one servo with external 5V supply, common GND
3. Write Phase 1 firmware: LEDC 50 Hz, sweep 0→180→0° on a 3 s loop
4. Verify servo motion, measure pulse on Fluke / scope
