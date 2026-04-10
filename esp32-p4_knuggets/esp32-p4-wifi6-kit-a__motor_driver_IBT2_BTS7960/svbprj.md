# esp32-p4-wifi6-kit-a__motor_driver_IBT2_BTS7960

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Module:** IBT-2 (BTS7960 dual H-bridge)
**Status:** IBT-2 PWM ramp test verified | ON/OFF field test verified | Two-speed random test verified | Rust SPA controller built | C6 Wi-Fi STA pending (needs slave flash)
**Anki-Ref:** `20260319_093229__BTS7960__dc_motor_drive`

## Project Folder Structure

```
esp32-p4-wifi6-kit-a__motor_driver_IBT2_BTS7960/
├── svbprj.md                          <- this file
├── anki_refs/                         <- Anki flashcard decks (.apkg exports)
├── pics/                              <- photos of physical builds
├── docs_about__motor_driver_IBT2_BTS7960/
│   ├── bts7960_usecases/              <- BTS7960 use-case diagrams + PNGs
│   ├── firmware_phase_1/              <- Phase 1: GPIO troubleshooting diagram
│   ├── firmware_phase_2__tst__motor_on_off/    <- Phase 2 TST: ON/OFF puzzle-piece test
│   │   ├── pin_connections__motor_on_off__20260324.drawio
│   │   └── firmware_phase_2__tst__motor_on_off__puzzle_piece_test__20260324.drawio
│   └── firmware_phase_2__tst__motor_pwd_FWD_REV/  <- Phase 2 TST: PWM ramp puzzle-piece test
│       ├── pin_connections__motor_pwm_FWD_REV__20260324.drawio
│       └── firmware_phase_2__tst__motor_pwm_FWD_REV__puzzle_piece_test__20260324.drawio
├── ...__free/                         <- student scaffolding (public)
└── ...__full/                         <- reference solution (private)
    ├── firmware_phase_1/              <- Phase 1: GPIO sweep + Fluke bench test (100Hz)
    ├── firmware_phase_2/              <- Phase 2: SPA Web Control (Wi-Fi AP + HTTP API)
    ├── firmware_phase_2__on_off_test/ <- Phase 2 TST: 3s ON/3s OFF motor cycle
    ├── firmware_phase_2__tst__motor_pwm_FWD_REV/  <- Phase 2 TST: PWM ramp 0->100->0%
    ├── firmware_phase_2__tst__motor_pwm_FWD_REV__two_speed_rnd/  <- Phase 2 TST: random 25%/50% bursts
    └── firmware_phase_3__tst__motor_pwd_FWD_REV__rs_app_cntrl/   <- Phase 3: Rust SPA + Wi-Fi STA
        ├── esp_idf_fw/               <- ESP-IDF firmware (serial JSON + Wi-Fi STA via C6)
        ├── rs_motor_cntrl/           <- Rust axum SPA on port 3044 (LAN-accessible)
        └── esp_idf_fw/main/wifi_secrets.h  <- Wi-Fi credentials (gitignored)
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

A "puzzle-piece" knugget for the ESP32-P4, designed to integrate into the
`esp32-p4-wifi6-kit-a__secure_wap_streamer` project:

**IBT-2 BTS7960** -- DC motor driver (43A, 6-27V, PWM speed control)

GPIO assignments are locked for puzzle-piece compatibility across all knuggets on this board.

See also: `esp32-p4-wifi6-kit-a__servo_driver_PCA9685` (companion puzzle-piece for pan/tilt servos)

### Puzzle-Piece Phase Testing

Each knugget is an input or output sensor/actuator tested in isolation. Phase testing
verifies raw I/O capability -- the specific application (use case) comes later.

| Phase | Purpose | Proves |
|-------|---------|--------|
| Phase 1 | GPIO + electrical | Pins output correct voltage (Fluke verified) |
| Phase 2 | Feature firmware | I/O firmware works (SPA, HTTP API, auto mode) |
| Phase 2 TST ON/OFF | Physical field test | Motor spins/stops reliably in real environment |
| Phase 2 TST PWM | PWM ramp test | Variable speed control works (0->25->50->75->100->down) |
| Phase 2 TST Two Speed Rnd | Random burst test | Random 25%/50% bursts with 1-20s pauses (pigeon scarer) |
| Phase 3 RS App Cntrl | Rust SPA controller | Motor control via browser (axum port 3044, LAN-accessible) |
| Phase 3 Wi-Fi STA | C6 joins home Wi-Fi | ESP32-P4 gets DHCP IP, controllable from tablet (BLOCKED: C6 needs esp_hosted slave flash) |
| Integration | All pieces combined | All I/O works together in secure_wap_streamer |

Use cases (what the I/O is used for) are documented separately in `bts7960_usecases/`.

## IBT-2 BTS7960 Motor Driver

- **Max current:** 43A continuous
- **Voltage:** 6-27V motor supply, 3.3V logic
- **Control:** PWM 25 kHz for speed, direction via RPWM/LPWM selection
- **Pins:** RPWM, LPWM, R_EN, L_EN, VCC, GND
- **Current wiring:** GPIO 25->RPWM, GPIO 32->R_EN, GPIO 22->L_EN (forward only, LPWM needs 4th GPIO)

### Control Logic

| RPWM | LPWM | Action    |
|------|------|-----------|
| PWM  | LOW  | Forward   |
| LOW  | PWM  | Reverse   |
| HIGH | HIGH | Brake     |
| LOW  | LOW  | Coast     |

## GPIO Pin Assignments (Waveshare 40-Pin Header)

### IBT-2 Motor Driver (Verified Working -- 2026-03-20)

| GPIO | Function | 40-Pin Header | Status |
|------|----------|-----------|--------|
| 25   | RPWM     | Left side | Confirmed |
| ??   | LPWM     | TBD       | Needs 4th GPIO (untested set) |
| 32   | R_EN     | Right side| Confirmed |
| 22   | L_EN     | Right side| Confirmed |

### GPIOs Reserved (DO NOT USE)

| GPIO             | Used By                   |
|------------------|---------------------------|
| 7, 8             | MI0802 I2C Bus 0 (SDA, SCL) |
| 14, 15, 16, 17   | SDIO D0-D3 (C6 Wi-Fi)    |
| 18               | SDIO CLK (C6 Wi-Fi)      |
| 19               | SDIO CMD (C6 Wi-Fi)      |
| 26, 27           | USB OTG D-/D+ -- DO NOT USE while USB connected |
| 54               | C6 Reset                  |

### GPIO Bench Test Results (2026-03-20)

Tested every GPIO on the 40-pin header with Fluke DMM. Many GPIOs labeled on the
board silkscreen do NOT output 3.3V when driven HIGH from firmware.

| GPIO | Side  | Fluke Result | Status |
|------|-------|-------------|--------|
| 2    | Left  | 1.8V (unstable) | **UNRELIABLE** -- has onboard pull |
| 3    | Left  | 0V | **DEAD** |
| 4    | Left  | 0V | **DEAD** |
| 5    | Left  | 0V | **DEAD** |
| 20   | Right | 0V | **DEAD** |
| 21   | Right | 0V | **DEAD** |
| 22   | Right | 3.3V cycles | **WORKS** |
| 23   | Right | 0V | **DEAD** |
| 25   | Left  | 3.3V cycles | **WORKS** |
| 26   | Right | 0V | Reserved USB D- |
| 27   | Right | 0V | Reserved USB D+ |
| 32   | Right | 3.3V cycles | **WORKS** |
| 33   | Right | 0V | **DEAD** |
| 46   | Right | 0V | **DEAD** |
| 47   | Right | 0V | **DEAD** |
| 48   | Right | 0V | **DEAD** |

**Only 3 confirmed working GPIOs: 22, 25, 32**

> **Note:** The silkscreen pin numbers may not match ESP32-P4 GPIO numbers.
> Further investigation needed -- may need Waveshare schematic to find true GPIO mapping.

### GPIO Budget Summary

- **Confirmed working:** 22 (Right), 25 (Left), 32 (Right)
- **Reserved:** 7, 8, 14-19, 26, 27, 54
- **Untested:** 24, 28, 29, 30, 31, 34, 36, 49, 50, 51, 52
- **Tested dead:** 2, 3, 4, 5, 20, 21, 23, 33, 46, 47, 48

## Wiring (Verified Working -- 2026-03-20)

### IBT-2 Motor Driver

```
ESP32-P4 (40-Pin Header)              IBT-2 8-Pin Connector
-----                                 -----
GPIO 25 (Left)  ---------------------->   Pin 7: RPWM (PWM speed)
                                       Pin 8: LPWM (disconnected -- forward only)
GPIO 32 (Right) ---------------------->   Pin 5: R_EN (enable)
GPIO 22 (Right) ---------------------->   Pin 6: L_EN (enable)
3V3     (Right) ---------------------->   Pin 1: VCC
GND     (both)  ---------------------->   Pin 2: GND
                                       Pin 3: R_IS (not connected -- current sense)
                                       Pin 4: L_IS (not connected -- current sense)

IBT-2 4-Pin Screw Terminal            External
-----                                 -----
Pin 1: B+  <--------------------------   Battery/PSU + (6-27V)
Pin 2: B-  <--------------------------   Battery/PSU - (GND, shared with ESP32)
Pin 3: M+  -------------------------->   Motor +
Pin 4: M-  -------------------------->   Motor -
```

### Bench Test Verification (2026-03-20)

| Test | Result |
|------|--------|
| Fluke on B+/B- | **20V** -- battery connected, power confirmed |
| Fluke on GPIO 25 (RPWM) | **3.3V cycling** -- firmware PWM output confirmed |
| Fluke on GPIO 32 (R_EN) | **3.3V cycling** -- enable pin confirmed |
| Fluke on GPIO 22 (L_EN) | **3.3V cycling** -- enable pin confirmed |
| Fluke on BTS7960 pin 5 (R_EN) | **3.3V cycling** -- signal reaching module confirmed |
| Fluke on BTS7960 pin 6 (L_EN) | **3.3V cycling** -- signal reaching module confirmed |
| Fluke on M+/M- (all signals ON) | **20V** -- H-bridge passing battery voltage to motor terminals |
| Serial monitor port | **/dev/ttyACM1** (not ACM0 -- ACM0 is flash/JTAG) |

### Troubleshooting Log

1. **GPIO 6 (original R_EN):** Not on 40-pin header -> reassigned to GPIO 27
2. **GPIO 27 (second attempt):** Reserved for USB D+ -> reads 0V -> reassigned to GPIO 32
3. **GPIO 4, 5 (original RPWM/LPWM):** Read 0V on header -> may be wrong GPIO mapping
4. **GPIO 33, 46, 48 (attempted RPWM):** All read 0V -> dead on this board
5. **GPIO 25:** Confirmed working -> assigned to RPWM
6. **GPIO 32:** Confirmed working -> assigned to R_EN
7. **GPIO 22:** Confirmed working -> assigned to L_EN
8. **LEDC PWM at 25kHz:** Works but Fluke reads peak (20V) not average -> lowered to 100Hz for bench testing
9. **Serial monitor:** Console output is on /dev/ttyACM1, flash via /dev/ttyACM0

## Diagrams

### General
- `esp32p4-to-ibt2-bts7960-wiring.drawio` -- detailed wiring diagram
- `knugget-puzzle-pieces-architecture.drawio` -- high-level puzzle-piece architecture with WebApp console
- `ibt2_pwm_motor_control.drawio` -- PWM motor control explained (H-bridge, duty cycle waveforms, control table)

### Phase Test Diagrams (puzzle-piece I/O verification)
- `firmware_phase_1/firmware_phase_1__gpio_troubleshooting__20260320.drawio` -- GPIO pin troubleshooting journey
- `firmware_phase_2__tst__motor_on_off/pin_connections__motor_on_off__20260324.drawio` -- pin connection diagram (3 GPIOs, forward only)
- `firmware_phase_2__tst__motor_on_off/firmware_phase_2__tst__motor_on_off__puzzle_piece_test__20260324.drawio` -- ON/OFF test architecture
- `firmware_phase_2__tst__motor_pwd_FWD_REV/pin_connections__motor_pwm_FWD_REV__20260324.drawio` -- pin connection diagram (4 GPIOs, LPWM TBD)
- `firmware_phase_2__tst__motor_pwd_FWD_REV/firmware_phase_2__tst__motor_pwm_FWD_REV__puzzle_piece_test__20260324.drawio` -- PWM ramp test architecture

### BTS7960 Application Use Cases (`bts7960_usecases/`)

| Diagram | Application |
|---------|-------------|
| `BTS7960__dc_motor_drive__20260319.drawio` | **DC Motor Drive (Overview)** -- parent diagram with 6 sub-applications |
| Sub-uses: | Robot head/arm, critter scarer, drawbridge/trap-door, feed dispenser, turntable, tank steering |
| `BTS7960__dc_motor_robot_drive__20260319.drawio` | Robot/rover tank steering -- 2x BTS7960 for differential drive |
| `BTS7960__drill_trigger_replacement__20260319.drawio` | Variable speed drill motor -- replaces mechanical trigger with PWM |
| `BTS7960__water_pump_control__20260319.drawio` | Irrigation pump -- soil moisture sensor -> auto pump control |
| `BTS7960__conveyor_belt__20260319.drawio` | Industrial conveyor -- fwd/rev, E-stop, soft start/stop |
| `BTS7960__winch_hoist__20260319.drawio` | Winch/hoist -- raise/lower with limit switches, overload detection |
| `BTS7960__electric_vehicle_throttle__20260319.drawio` | E-bike/go-kart throttle -- analog input, regen braking, speed limiting |
| `BTS7960__electric_strike_door_lock__20260319.drawio` | Door lock -- BTS7960 vs PCA9685 vs Relay comparison, electric strike specs |

Regenerate PNGs: `drawio --export --format png --scale 2 --output X.png X.drawio`

## Build & Flash

```bash
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh

# Choose firmware variant:
cd rs_riscvml__...__full/firmware_phase_2__on_off_test       # simple ON/OFF
cd rs_riscvml__...__full/firmware_phase_2__tst__motor_pwm_FWD_REV  # PWM ramp
cd rs_riscvml__...__full/firmware_phase_2                     # SPA web control

idf.py build
idf.py -p /dev/ttyACM0 flash
```

**USB ports:** Flash on `/dev/ttyACM0`. Serial monitor on `/dev/ttyACM0` or `/dev/ttyACM1` (board-dependent -- CH9102 chip provides single port).

## Firmware Variants

### firmware_phase_1 -- GPIO Sweep + Bench Test
- GPIO sweep firmware to find working pins (Fluke DMM verification)
- LEDC PWM at 100Hz (Fluke readable), voltage ramp 1V->15V->1V
- Result: only 3 working GPIOs found (22, 25, 32)

### firmware_phase_2 -- SPA Web Control
- Wi-Fi AP via esp_hosted (SDIO to C6), SSID: `RISCVML-Motor`
- Embedded SPA (index.html in flash), HTTP REST API
- Endpoints: `GET /api/status`, `POST /api/motor`, `POST /api/auto`
- Auto mode with random burst pattern (FreeRTOS task)
- 25 kHz PWM, 10-bit resolution

### firmware_phase_2__on_off_test -- Motor ON/OFF (Field Test verified)
- Minimal: 3s ON at 50% / 3s OFF, repeating forever
- No Wi-Fi -- standalone motor cycling
- Verified working on balcony deployment (2026-03-23)

### firmware_phase_2__tst__motor_pwm_FWD_REV -- PWM Ramp (Field Test verified)
- PWM ramp: 0% -> 25% -> 50% -> 75% -> 100% -> back down
- 3 seconds per step, 25 kHz PWM
- Verified working -- variable speed confirmed (2026-03-24)
- Forward only (LPWM disconnected -- reverse requires 4th GPIO)

### firmware_phase_2__tst__motor_pwm_FWD_REV__two_speed_rnd -- Two Speed Random (2026-03-27)
- Pigeon scarer mode: randomly picks 25% or 50% speed
- Motor runs 2-5 seconds, then off for 1-20 seconds random pause
- Uses ESP32 hardware RNG (`esp_random()`)
- Standalone -- no Wi-Fi, no serial commands, just runs forever
- 25 kHz PWM, forward only

### firmware_phase_3__tst__motor_pwd_FWD_REV__rs_app_cntrl -- Rust SPA Controller (2026-03-27)
- **ESP-IDF firmware:** Serial JSON command listener + Wi-Fi STA via C6 esp_hosted
  - Commands: `{"cmd":"speed","val":50}`, `{"cmd":"on"}`, `{"cmd":"off"}`, `{"cmd":"status"}`
  - Responds with JSON: `{"speed":50,"on":true,"voltage":"10.0V","ip":"192.168.x.x"}`
  - Heartbeat: prints human-readable status every 30 seconds
  - Wi-Fi STA: joins home network via C6 SDIO, prints DHCP IP
- **Rust SPA (rs_motor_cntrl):** axum web server on port 3044
  - Embedded HTML SPA (include_str!), accessible from any LAN device
  - ON/OFF power buttons, PWM speed slider (0-100%), preset buttons
  - Serial mode: connects to ESP32-P4 via USB `/dev/ttyACM0`
  - Wi-Fi mode: enter ESP32's IP, commands go directly to device
  - BLOCKED: C6 needs esp_hosted slave firmware flashed first
  - C6 USB pads are `V D- D+ G` (no connector, no BOOT button -- needs solder job)

## Use Case: Variable Speed Drill Trigger Control

The IBT-2 can replace a drill's trigger to provide electronic speed control from the ESP32-P4.

### Confirmed: Brushed Drill (19V Battery)

Fluke measurements on the drill trigger output:
- Trigger released: **0V DC**
- Light press: **~1V DC**
- Full press: **~19V DC**

This confirms a **brushed motor with a variable-voltage trigger** (not a low-voltage control signal).
The IBT-2 replaces the trigger -- ESP32-P4 PWM duty cycle maps directly to motor speed.

| PWM Duty | Motor Voltage | Equivalent Trigger |
|----------|---------------|-------------------|
| 0%       | 0V            | Released          |
| 5%       | ~1V           | Light press       |
| 50%      | ~9.5V         | Half pull         |
| 100%     | ~19V          | Full press        |

### Wiring (Drill Trigger Replacement)

```
Drill Battery (19V)              IBT-2              Drill Motor
-----                            -----              -----
    Battery +  -------->  VIN                 B+  -------->  Motor +
    Battery -  -------->  GND                 B-  -------->  Motor -

ESP32-P4 (40-Pin Header)            IBT-2
-----                                -----
GPIO 25 (Left)  ----------------->  RPWM  (speed 0-100%)
                                    LPWM  (disconnected -- forward only)
GPIO 32 (Right) ----------------->  R_EN  (HIGH -- enable)
GPIO 22 (Right) ----------------->  L_EN  (HIGH -- enable)
3V3     (Right) ----------------->  VCC
GND     (both)  ----------------->  GND   (common with drill battery -)
```

Disconnect the two wires from the trigger output to the motor. Connect them to B+/B- instead.
Forward-only operation: RPWM = PWM, LPWM = LOW, both enables HIGH.

### Quick Bench Test (No ESP32-P4 Required)

To verify the IBT-2 can spin the drill motor before writing firmware:

```
1. Wire VIN/GND <-- Drill battery (19V)
2. Wire B+/B-   --> Drill motor leads
3. Wire VCC     <-- 3.3V (or AA battery 1.5V)
4. Wire R_EN    <-- jumper to VCC
5. Wire L_EN    <-- jumper to VCC
6. Wire LPWM    <-- jumper to GND
7. Touch RPWM   <-- briefly to VCC -> motor should spin
```

BTS7960 logic threshold is ~1.2V, so even a 1.5V AA battery works as a logic source.

### Power Source Option: Waveshare UPS Power Module (C)

The UPS Module (C) with 3S 21700 cells (9V-12.6V) can power the IBT-2 VIN for light motor loads (<2A).
For heavy loads (drill motors at 5A+), use the drill's own battery pack directly.

## Future Integration (Secure WAP Streamer)

This knugget will be integrated into the `secure_wap_streamer` as an ESP-IDF component:
- `/api/motor` -- POST speed (0-100%), direction (fwd/rev/brake/coast)
- WebApp console with sliders for motor speed
- Combined with `esp32-p4-wifi6-kit-a__servo_driver_PCA9685` for pan/tilt
- All controlled via HTTPS over the "crittercam" Wi-Fi AP
