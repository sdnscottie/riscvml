# rs_riscvml__esp32-p4-wifi6-kit-a__motor_driver_IBT2_BTS7960__servo_driver_PCA9685

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Modules:** IBT-2 (BTS7960 dual H-bridge) + PCA9685 (16-ch I2C servo driver)
**Status:** IBT-2 firmware builds ✓ · PCA9685 firmware TODO
**Anki-Ref:** `20260319_093229__BTS7960__dc_motor_drive`

## Project Folder Structure

```
rs_riscvml__...__motor_driver_IBT2_BTS7960__servo_driver_PCA9685/
├── svbprj.md                          ← this file
├── anki_refs/                         ← Anki flashcard decks (.apkg exports)
├── pics/                              ← photos of physical builds
├── docs_about__motor_driver_.../      ← diagrams & docs
│   ├── bts7960_usecases/              ← BTS7960 use-case diagrams + PNGs
│   └── pca9685_usecases/              ← PCA9685 use-case diagrams + PNGs
├── ...__free/                         ← student scaffolding (public)
└── ...__full/                         ← reference solution (private)
```

### Anki-Ref Convention

Each project/diagram has an Anki flashcard reference linking it to the Anki spaced-repetition deck:

```
Format: <YYYYMMDD>_<HHMMSS>__<component>__<use_case>
Example: 20260319_093229__BTS7960__dc_motor_drive
```

- The `anki_refs/` folder holds exported Anki deck files (`.apkg`)
- The `pics/` folder holds build photos used on Anki card fronts/backs
- Each drawio diagram includes the Anki-Ref as a black label bar
- The timestamp is the flashcard creation timestamp (not the diagram timestamp)

## Overview

Two "puzzle-piece" knuggets for the ESP32-P4, designed to integrate into the
`rs_riscvml__esp32-p4-wifi6-kit-a__secure_wap_streamer` project:

1. **IBT-2 BTS7960** — DC motor driver (43A, 6–27V, PWM speed control)
2. **PCA9685** — I2C servo driver (16 channels, 12-bit PWM, pan/tilt for 2 servos)

GPIO assignments are locked for puzzle-piece compatibility across all knuggets on this board.

## IBT-2 BTS7960 Motor Driver

- **Max current:** 43A continuous
- **Voltage:** 6–27V motor supply, 3.3V logic
- **Control:** PWM 25 kHz for speed, direction via RPWM/LPWM selection
- **Pins:** RPWM, LPWM, R_EN, L_EN, VCC, GND

### Control Logic

| RPWM | LPWM | Action    |
|------|------|-----------|
| PWM  | LOW  | ▶ Forward |
| LOW  | PWM  | ◀ Reverse |
| HIGH | HIGH | ⏹ Brake   |
| LOW  | LOW  | ⏸ Coast   |

## PCA9685 Servo Driver

- **Interface:** I2C, default address 0x40
- **Channels:** 16 PWM outputs, 12-bit resolution
- **Servo frequency:** 50 Hz (standard hobby servos)
- **Used channels:** CH0 = Pan, CH1 = Tilt
- **Power:** Separate 5–6V servo supply via V+ screw terminal

## GPIO Pin Assignments (Waveshare 40-Pin Header)

### IBT-2 Motor Driver

| GPIO | Function | 40-Pin Header |
|------|----------|-----------|
| 4    | RPWM     | Top row   |
| 5    | LPWM     | Top row   |
| 32   | R_EN     | Right side|
| 22   | L_EN     | Top row   |

### PCA9685 Servo Driver (I2C Bus 1)

| GPIO | Function | 40-Pin Header |
|------|----------|-----------|
| SDA  | SDA      | Left side (labeled SDA pad) |
| SCL  | SCL      | Left side (labeled SCL pad) |

### GPIOs Reserved by Other Puzzle Pieces (DO NOT USE)

| GPIO             | Used By                   |
|------------------|---------------------------|
| 7, 8             | MI0802 I2C Bus 0 (SDA, SCL) |
| 14, 15, 16, 17   | SDIO D0–D3 (C6 Wi-Fi)    |
| 18               | SDIO CLK (C6 Wi-Fi)      |
| 19               | SDIO CMD (C6 Wi-Fi)      |
| 54               | C6 Reset                  |

### GPIO Budget Summary

- **Used:** 4, 5, 7, 8, 14–19, 20, 21, 22, 32, 54
- **Reserved (USB OTG):** 26 (D-), 27 (D+) — DO NOT USE while USB connected
- **Available:** 0, 1, 2, 3, 23, 24, 25, 28, 29, 30, 31, 33, 36, 46, 47, 48

## Wiring

### IBT-2 Motor Driver

```
ESP32-P4 (40-Pin Header)         IBT-2 Module
─────────────────────         ────────────
GPIO 4  ──────────────────►   RPWM
GPIO 5  ──────────────────►   LPWM
GPIO 32  ──────────────────►   R_EN
GPIO 22 ──────────────────►   L_EN
3V3     ──────────────────►   VCC
GND     ──────────────────►   GND

Motor Power (separate supply!)
─────────────────────────────
B+  ◄───── Motor +
B-  ◄───── Motor -
VIN ◄───── 6–27V supply +
GND ◄───── 6–27V supply - (shared with ESP32 GND)
```

### PCA9685 Servo Driver

```
ESP32-P4 (40-Pin Header)         PCA9685 Module
─────────────────────         ──────────────
GPIO 21 ──────────────────►   SDA
GPIO 20 ──────────────────►   SCL
3V3     ──────────────────►   VCC
GND     ──────────────────►   GND

Servo Outputs
─────────────
CH0     ──────────────────►   Servo 1 (Pan)  signal wire
CH1     ──────────────────►   Servo 2 (Tilt) signal wire
V+      ◄───── 5–6V servo supply (separate from motor supply)
```

**Important:** All GND lines must be connected (ESP32-P4, motor supply, servo supply = common ground).

## Diagrams

- `docs_about__motor_driver_IBT2_BTS7960__servo_driver_PCA9685/esp32p4-to-ibt2-bts7960-wiring.drawio` — detailed wiring diagram
- `docs_about__motor_driver_IBT2_BTS7960__servo_driver_PCA9685/knugget-puzzle-pieces-architecture.drawio` — high-level puzzle-piece architecture with WebApp console
- `docs_about__motor_driver_IBT2_BTS7960__servo_driver_PCA9685/ibt2_pwm_motor_control.drawio` — PWM motor control explained (H-bridge, duty cycle waveforms, control table)

### BTS7960 Application Use Cases (`bts7960_usecases/`)

| Diagram | Application |
|---------|-------------|
| `BTS7960__dc_motor_drive__20260319.drawio` | **DC Motor Drive (Overview)** — parent diagram with 6 sub-applications |
| ↳ Sub-uses: | Robot head/arm, critter scarer, drawbridge/trap-door, feed dispenser, turntable, tank steering |
| `BTS7960__dc_motor_robot_drive__20260319.drawio` | Robot/rover tank steering — 2x BTS7960 for differential drive |
| `BTS7960__drill_trigger_replacement__20260319.drawio` | Variable speed drill motor — replaces mechanical trigger with PWM |
| `BTS7960__water_pump_control__20260319.drawio` | Irrigation pump — soil moisture sensor → auto pump control |
| `BTS7960__conveyor_belt__20260319.drawio` | Industrial conveyor — fwd/rev, E-stop, soft start/stop |
| `BTS7960__winch_hoist__20260319.drawio` | Winch/hoist — raise/lower with limit switches, overload detection |
| `BTS7960__electric_vehicle_throttle__20260319.drawio` | E-bike/go-kart throttle — analog input, regen braking, speed limiting |
| `BTS7960__electric_strike_door_lock__20260319.drawio` | Door lock — BTS7960 vs PCA9685 vs Relay comparison, electric strike specs |

### PCA9685 Application Use Cases (`pca9685_usecases/`)

| Diagram | Application |
|---------|-------------|
| `PCA9685__camera_pan_tilt__20260319.drawio` | Camera pan/tilt mount — bird detection tracking, crittercam integration |
| `PCA9685__robotic_arm_4dof__20260319.drawio` | 4-DOF robotic arm — base, shoulder, elbow, gripper with IK |
| `PCA9685__hexapod_walking_robot__20260319.drawio` | Hexapod — 2x PCA9685 daisy-chained, tripod gait pattern |
| `PCA9685__solar_panel_tracker__20260319.drawio` | Dual-axis solar tracker — LDR quadrant sensing, sun-following |
| `PCA9685__automated_greenhouse__20260319.drawio` | Greenhouse automation — vents, louvers, water valves (9 servos) |
| `PCA9685__led_lighting_controller__20260319.drawio` | 16-ch LED dimmer — grow lights, MOSFET drivers, 12-bit dimming |
| `PCA9685__servo_door_lock_multi__20260319.drawio` | Multi-door servo lock — 16 locks from one module, retrofit deadbolts |

Regenerate PNGs: `drawio --export --format png --scale 2 --output X.png X.drawio`

## Build & Flash

```bash
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh
cd rs_riscvml__esp32-p4-wifi6-kit-a__motor_driver_IBT2_BTS7960__servo_driver_PCA9685__full/esp_idf_ws
idf.py build
idf.py -p /dev/ttyACM0 flash monitor   # Ctrl+] to exit
```

## Firmware Features (IBT-2 — implemented)

- PWM motor control via LEDC peripheral (25 kHz, 10-bit resolution)
- Forward / reverse / brake / coast commands
- Variable speed (0–100%)
- Demo task: ramp forward, brake, ramp reverse, coast, repeat

## Firmware Features (PCA9685 — TODO)

- I2C driver for PCA9685 at 0x40 on I2C Bus 1 (GPIO 20, 21)
- Servo angle control (0–180°) on CH0 (pan) and CH1 (tilt)
- Pulse width mapping: 1ms (0°) → 2ms (180°) at 50Hz

## Use Case: Variable Speed Drill Trigger Control

The IBT-2 can replace a drill's trigger to provide electronic speed control from the ESP32-P4.

### Confirmed: Brushed Drill (19V Battery)

Fluke measurements on the drill trigger output:
- Trigger released: **0V DC**
- Light press: **~1V DC**
- Full press: **~19V DC**

This confirms a **brushed motor with a variable-voltage trigger** (not a low-voltage control signal).
The IBT-2 replaces the trigger — ESP32-P4 PWM duty cycle maps directly to motor speed.

| PWM Duty | Motor Voltage | Equivalent Trigger |
|----------|---------------|-------------------|
| 0%       | 0V            | Released          |
| 5%       | ~1V           | Light press       |
| 50%      | ~9.5V         | Half pull         |
| 100%     | ~19V          | Full press        |

### Wiring (Drill Trigger Replacement)

```
Drill Battery (19V)              IBT-2              Drill Motor
━━━━━━━━━━━━━━━━━━━    ━━━━━━━━━━━━━━━━━    ━━━━━━━━━━━━━━
    Battery +  ──────────►  VIN                 B+  ──────────►  Motor +
    Battery -  ──────────►  GND                 B-  ──────────►  Motor -

ESP32-P4 (40-Pin Header)            IBT-2
━━━━━━━━━━━━━━━━━━━━    ━━━━━━━━━━━━━━━━━
GPIO 4   ────────────────────►  RPWM  (speed 0–100%)
GPIO 5   ────────────────────►  LPWM  (LOW — forward only)
GPIO 32   ────────────────────►  R_EN  (HIGH — enable)
GPIO 22  ────────────────────►  L_EN  (HIGH — enable)
3V3      ────────────────────►  VCC
GND      ────────────────────►  GND   (common with drill battery -)
```

Disconnect the two wires from the trigger output to the motor. Connect them to B+/B- instead.
Forward-only operation: RPWM = PWM, LPWM = LOW, both enables HIGH.

### Quick Bench Test (No ESP32-P4 Required)

To verify the IBT-2 can spin the drill motor before writing firmware:

```
1. Wire VIN/GND ◄── Drill battery (19V)
2. Wire B+/B-   ──► Drill motor leads
3. Wire VCC     ◄── 3.3V (or AA battery 1.5V)
4. Wire R_EN    ◄── jumper to VCC
5. Wire L_EN    ◄── jumper to VCC
6. Wire LPWM    ◄── jumper to GND
7. Touch RPWM   ◄── briefly to VCC → motor should spin
```

BTS7960 logic threshold is ~1.2V, so even a 1.5V AA battery works as a logic source.

### Power Source Option: Waveshare UPS Power Module (C)

The UPS Module (C) with 3S 21700 cells (9V–12.6V) can power the IBT-2 VIN for light motor loads (<2A).
For heavy loads (drill motors at 5A+), use the drill's own battery pack directly.

## Future Integration (Secure WAP Streamer)

Both knuggets will be integrated into the `secure_wap_streamer` as ESP-IDF components:
- `/api/motor` — POST speed (0–100%), direction (fwd/rev/brake/coast)
- `/api/servo` — POST pan (0–180°), tilt (0–180°)
- WebApp console with sliders for motor speed and servo pan/tilt
- All controlled via HTTPS over the "crittercam" Wi-Fi AP
