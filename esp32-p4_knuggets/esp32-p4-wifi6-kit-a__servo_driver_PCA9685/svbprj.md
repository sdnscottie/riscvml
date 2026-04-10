# esp32-p4-wifi6-kit-a__servo_driver_PCA9685

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** PCA9685 (16-ch I2C servo driver)
**Status:** Use case diagrams complete | Firmware TODO | Wiring untested
**Anki-Ref:** TBD

## Project Folder Structure

```
esp32-p4-wifi6-kit-a__servo_driver_PCA9685/
├── svbprj.md                          <- this file
├── anki_refs/                         <- Anki flashcard decks (.apkg exports)
├── pics/                              <- photos of physical builds
├── docs_about__servo_driver_PCA9685/
│   └── pca9685_usecases/              <- PCA9685 use-case diagrams + PNGs
├── ...__free/                         <- student scaffolding (public)
│   └── esp_idf_ws/main/              <- ESP-IDF workspace stub
└── ...__full/                         <- reference solution (private)
    └── firmware_phase_1/main/         <- Phase 1: I2C + basic servo control (TODO)
```

### Anki-Ref Convention

Each project/diagram has an Anki flashcard reference linking it to the Anki spaced-repetition deck:

```
Format: <YYYYMMDD>_<HHMMSS>__<component>__<use_case>
Example: 20260319_093229__PCA9685__camera_pan_tilt
```

- The `anki_refs/` folder holds exported Anki deck files (`.apkg`)
- The `pics/` folder holds build photos used on Anki card fronts/backs
- Each drawio diagram includes the Anki-Ref as a black label bar
- The timestamp is the flashcard creation timestamp (not the diagram timestamp)

## Overview

A "puzzle-piece" knugget for the ESP32-P4, designed to integrate into the
`esp32-p4-wifi6-kit-a__secure_wap_streamer` project:

**PCA9685** -- I2C servo driver (16 channels, 12-bit PWM, pan/tilt for 2 servos)

GPIO assignments are locked for puzzle-piece compatibility across all knuggets on this board.

See also: `esp32-p4-wifi6-kit-a__motor_driver_IBT2_BTS7960` (companion puzzle-piece for DC motor drive)

### Puzzle-Piece Phase Testing

Each knugget is an input or output sensor/actuator tested in isolation. Phase testing
verifies raw I/O capability -- the specific application (use case) comes later.

| Phase | Purpose | Proves |
|-------|---------|--------|
| Phase 1 | I2C + basic servo | PCA9685 responds on I2C, servo sweeps 0-180 degrees |
| Phase 2 | Feature firmware | SPA web control for pan/tilt, HTTP API |
| Phase 2 TST | Physical field test | Servos move reliably under load (camera mount) |
| Integration | All pieces combined | All I/O works together in secure_wap_streamer |

Use cases (what the I/O is used for) are documented separately in `pca9685_usecases/`.

## PCA9685 Servo Driver

- **Interface:** I2C, default address 0x40
- **Channels:** 16 PWM outputs, 12-bit resolution
- **Servo frequency:** 50 Hz (standard hobby servos)
- **Used channels:** CH0 = Pan, CH1 = Tilt
- **Power:** Separate 5-6V servo supply via V+ screw terminal

## GPIO Pin Assignments (Waveshare 40-Pin Header)

### PCA9685 Servo Driver (I2C -- Untested)

| GPIO | Function | 40-Pin Header |
|------|----------|-----------|
| 21   | SDA      | Right side |
| 20   | SCL      | Right side |

> **Note:** GPIO 20 and 21 showed 0V in the motor driver GPIO bench test (2026-03-20).
> These may be dead on the 40-pin header or may need alternate I2C configuration.
> Investigation needed before PCA9685 firmware work begins.

### GPIOs Reserved (DO NOT USE)

| GPIO             | Used By                   |
|------------------|---------------------------|
| 7, 8             | MI0802 I2C Bus 0 (SDA, SCL) |
| 14, 15, 16, 17   | SDIO D0-D3 (C6 Wi-Fi)    |
| 18               | SDIO CLK (C6 Wi-Fi)      |
| 19               | SDIO CMD (C6 Wi-Fi)      |
| 22, 25, 32       | IBT-2 motor driver (RPWM, R_EN, L_EN) |
| 26, 27           | USB OTG D-/D+ -- DO NOT USE while USB connected |
| 54               | C6 Reset                  |

## Wiring (Planned -- Not Yet Tested)

### PCA9685 Servo Driver

```
ESP32-P4 (40-Pin Header)         PCA9685 Module
-----                             -----
GPIO 21 ---------------------->   SDA
GPIO 20 ---------------------->   SCL
3V3     ---------------------->   VCC
GND     ---------------------->   GND

Servo Outputs
-----
CH0     ---------------------->   Servo 1 (Pan)  signal wire
CH1     ---------------------->   Servo 2 (Tilt) signal wire
V+      <----- 5-6V servo supply (separate from motor supply)
```

**Important:** All GND lines must be connected (ESP32-P4, motor supply, servo supply = common ground).

## Diagrams

### PCA9685 Application Use Cases (`pca9685_usecases/`)

| Diagram | Application |
|---------|-------------|
| `PCA9685__camera_pan_tilt__20260319.drawio` | Camera pan/tilt mount -- bird detection tracking, crittercam integration |
| `PCA9685__robotic_arm_4dof__20260319.drawio` | 4-DOF robotic arm -- base, shoulder, elbow, gripper with IK |
| `PCA9685__hexapod_walking_robot__20260319.drawio` | Hexapod -- 2x PCA9685 daisy-chained, tripod gait pattern |
| `PCA9685__solar_panel_tracker__20260319.drawio` | Dual-axis solar tracker -- LDR quadrant sensing, sun-following |
| `PCA9685__automated_greenhouse__20260319.drawio` | Greenhouse automation -- vents, louvers, water valves (9 servos) |
| `PCA9685__led_lighting_controller__20260319.drawio` | 16-ch LED dimmer -- grow lights, MOSFET drivers, 12-bit dimming |
| `PCA9685__servo_door_lock_multi__20260319.drawio` | Multi-door servo lock -- 16 locks from one module, retrofit deadbolts |

Regenerate PNGs: `drawio --export --format png --scale 2 --output X.png X.drawio`

## Firmware (TODO)

### firmware_phase_1 -- I2C + Basic Servo Control
- I2C driver for PCA9685 at 0x40 on I2C Bus 1 (GPIO 20, 21)
- Servo angle control (0-180 degrees) on CH0 (pan) and CH1 (tilt)
- Pulse width mapping: 1ms (0 degrees) -> 2ms (180 degrees) at 50Hz
- First step: verify GPIO 20/21 work for I2C on this board

## Build & Flash

```bash
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh

cd rs_riscvml__...__full/firmware_phase_1

idf.py build
idf.py -p /dev/ttyACM0 flash
```

**USB ports:** Flash on `/dev/ttyACM0`. Serial monitor on `/dev/ttyACM0` or `/dev/ttyACM1` (board-dependent -- CH9102 chip provides single port).

## Future Integration (Secure WAP Streamer)

This knugget will be integrated into the `secure_wap_streamer` as an ESP-IDF component:
- `/api/servo` -- POST pan (0-180 degrees), tilt (0-180 degrees)
- WebApp console with sliders for servo pan/tilt
- Combined with `esp32-p4-wifi6-kit-a__motor_driver_IBT2_BTS7960` for DC motor
- All controlled via HTTPS over the "crittercam" Wi-Fi AP
