# rs_riscvml__esp32-p4-wifi6-kit-a__motor_driver_IBT2_BTS7960__servo_driver_PCA9685

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Board:** Waveshare ESP32-P4-WIFI6 Kit A
**Modules:** IBT-2 (BTS7960 dual H-bridge) + PCA9685 (16-ch I2C servo driver)
**Status:** IBT-2 firmware builds ✓ · PCA9685 firmware TODO

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

## GPIO Pin Assignments (Waveshare P6 Header)

### IBT-2 Motor Driver

| GPIO | Function | P6 Header |
|------|----------|-----------|
| 4    | RPWM     | Top row   |
| 5    | LPWM     | Top row   |
| 6    | R_EN     | Bottom row|
| 22   | L_EN     | Top row   |

### PCA9685 Servo Driver (I2C Bus 1)

| GPIO | Function | P6 Header |
|------|----------|-----------|
| 21   | SDA      | Bottom row|
| 20   | SCL      | Bottom row|

### GPIOs Reserved by Other Puzzle Pieces (DO NOT USE)

| GPIO             | Used By                   |
|------------------|---------------------------|
| 7, 8             | MI0802 I2C Bus 0 (SDA, SCL) |
| 14, 15, 16, 17   | SDIO D0–D3 (C6 Wi-Fi)    |
| 18               | SDIO CLK (C6 Wi-Fi)      |
| 19               | SDIO CMD (C6 Wi-Fi)      |
| 54               | C6 Reset                  |

### GPIO Budget Summary

- **Used:** 4, 5, 6, 7, 8, 14–19, 20, 21, 22, 54
- **Available:** 0, 1, 2, 3, 23, 24, 25, 26, 27, 32, 33, 36, 45, 46, 47, 48, 53

## Wiring

### IBT-2 Motor Driver

```
ESP32-P4 (P6 Header)         IBT-2 Module
─────────────────────         ────────────
GPIO 4  ──────────────────►   RPWM
GPIO 5  ──────────────────►   LPWM
GPIO 6  ──────────────────►   R_EN
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
ESP32-P4 (P6 Header)         PCA9685 Module
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

## Future Integration (Secure WAP Streamer)

Both knuggets will be integrated into the `secure_wap_streamer` as ESP-IDF components:
- `/api/motor` — POST speed (0–100%), direction (fwd/rev/brake/coast)
- `/api/servo` — POST pan (0–180°), tilt (0–180°)
- WebApp console with sliders for motor speed and servo pan/tilt
- All controlled via HTTPS over the "crittercam" Wi-Fi AP
