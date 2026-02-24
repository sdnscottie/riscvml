# ESP32-P4 RISC-V Reference

## Core Architecture

The ESP32-P4 is Espressif's high-performance RISC-V MCU, a significant step up from the ESP32-C3/C6 line. It uses a "big-little" dual-subsystem design:

### HP (High-Performance) System
- **CPU:** Dual-core 32-bit RISC-V @ up to 400 MHz
- **ISA:** RV32IMAFCZc (Integer, Multiply, Atomic, Float, Compressed)
- **Custom Extensions:**
  - `Xhwlp` — Hardware loop optimization (reduces instructions in loop bodies)
  - `Xai` — AI and DSP extension for accelerated ML/signal processing operations
- **FPU:** Single-precision floating point
- **Memory:** 768 KB on-chip SRAM, 8 KB zero-wait TCM RAM
- **External:** Up to 32 MB PSRAM, external flash support

### LP (Low-Power) System
- **CPU:** Single-core 32-bit RISC-V @ up to 40 MHz
- **Memory:** Dedicated SRAM, ROM, and peripherals
- **Purpose:** Always-on housekeeping, RTC, wake logic — HP cores can sleep while LP core maintains system state

## Key Differentiator: No Built-In Wireless

The ESP32-P4 is the first ESP32 without an RF radio. No Wi-Fi, no Bluetooth, no Zigbee on-chip. This is by design — it positions against STM32F7/H7 and NXP i.MX RT Cortex-M7 parts at lower cost.

**Wireless connectivity** is achieved by pairing with a companion chip:
- ESP32-C6 (Wi-Fi 6 + BLE 5 + Thread/Zigbee) via SPI/SDIO
- ESP-Hosted or ESP-AT firmware on the companion
- Wired Ethernet (10/100 MAC) is also available natively

## Multimedia & HMI Hardware

This is where the P4 really shines for an MCU-class device:

- **Display:** MIPI-DSI interface, parallel RGB, up to 1080p
- **Camera:** MIPI-CSI with integrated Image Signal Processor (ISP), up to 1080p
- **H.264 Encoder:** Hardware, 1080p @ 30fps
- **JPEG Codec:** Up to 4K stills, 1080p @ 40fps encode/decode
- **PPA:** Pixel Processing Accelerator — hardware scaling, rotation, blending
- **2D-DMA:** For GUI framebuffer operations
- **Touch:** Capacitive touch inputs
- **Audio:** Speech recognition support, I2S interfaces

## Peripheral Interfaces

- **55 programmable GPIOs** (major increase over C3/C6)
- **USB:** OTG 2.0 High-Speed + Full-Speed PHYs, plus USB 1.1 Serial/JTAG
- **Ethernet:** 10/100 MAC
- **SDIO:** Host 3.0
- **Serial:** 5x UART (RS232/RS485/IrDA), SPI (1/2/4/8-bit modes), 2x I2C (up to 800 kbit/s), I3C (host + slave)
- **Motor/LED:** MCPWM, LED PWM, RMT
- **ADC/DAC:** Analog I/O
- **I2S:** Audio codec interface
- **TWAI:** CAN-compatible bus

## Security

- Secure Boot
- Flash Encryption (XTS-AES for flash and PSRAM)
- Cryptographic accelerators: RSA, ECDSA, HMAC, SHA
- TRNG (True Random Number Generator)
- Digital Signature Peripheral — private keys generated on-chip, never visible in plaintext
- Key Management Unit
- Access Permission Management and Privilege Separation

## Power Management

- **Active Mode:** Full CPU + all peripherals
- **Light-sleep Mode:** CPU paused, selective peripheral shutdown
- **Deep-sleep Mode:** CPU and most peripherals off, LP memory + select peripherals remain
- Multiple power domains (HP, LP, analog) with undervoltage monitoring

## Development

- **Framework:** ESP-IDF (same toolchain as C3/C6 — Rust support via esp-rs)
- **Dev Boards:**
  - ESP32-P4-Function-EV-Board (7" touchscreen, 2MP MIPI camera, ESP32-C6 companion)
  - Waveshare ESP32-P4-NANO (compact, PoE-capable, ESP32-C6-MINI module)
  - **Olimex ESP32-P4-PC** — the board used in this workspace
  - Various third-party boards emerging
- **Software:** LVGL v8/v9 ports available, ESP_Brookesia (Android-like UI framework)

## Workspace Setup

### Hardware
- **Board:** Olimex ESP32-P4-PC
- **Future firmware:** Rust (via esp-rs)

### ESP-IDF Installation
- **Path:** `~/Dropbox/scottsoft_sdn/esp-idf/`
- **Version:** v6.0-dev (development branch)
- **Toolchain:** RISC-V GCC 15.1.0 (`riscv32-esp-elf`)
- **Activate:** `source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh`

### Project: hello_p4
- **Path:** `esp32-p4/hello_p4/`
- **Description:** Minimal ESP-IDF hello world — prints chip info, heap size, loops with delay
- **Target:** esp32p4
- **Build:** `idf.py build` (confirmed working, 197 KB binary, 81% partition free)
- **Flash:** `idf.py -p /dev/ttyACM0 flash`
- **Monitor:** `idf.py -p /dev/ttyACM0 monitor` (requires interactive TTY — run from a real terminal)

### First Flash: Complete Setup Guide

**See also:** `esp32-p4-first-steps.drawio` — visual flowchart of this entire process.

#### Step 0: Linux Prerequisites
```bash
sudo apt install git wget flex bison gperf python3 python3-venv \
  cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0
```

#### Step 1: Serial Port Permissions
```bash
sudo usermod -aG dialout $USER
# Logout and login again for group membership to take effect
```

#### Step 2: Connect the Board
Plug in the Olimex ESP32-P4-PC via USB-C.

#### Step 3: Find the USB Device
```bash
ls /dev/ttyACM* /dev/ttyUSB*
# — or —
dmesg | grep tty
```
The Olimex ESP32-P4-PC uses the built-in **USB-Serial/JTAG** peripheral, which enumerates as an **ACM** device — typically `/dev/ttyACM0`. It will **NOT** appear as `/dev/ttyUSB*`.

#### Step 4: Activate ESP-IDF Environment
```bash
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh
```
This adds `idf.py`, the RISC-V toolchain, and the Python venv to your current shell `PATH`. **This must be run in every new terminal session** — it does not persist.

**If you get `idf.py: command not found`**, you forgot this step.

**Tip:** Add a shortcut to `~/.bashrc`:
```bash
alias get_idf='source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh'
```
Then just type `get_idf` before working.

#### Step 5: Configure Flash Size (One-Time Fix)
The default ESP-IDF project template assumes 2 MB flash, but the Olimex board has **16 MB**. Without this fix, you get a boot warning:
> `Detected size(16384k) larger than the size in the binary image header(2048k)`

Ensure `sdkconfig.defaults` contains:
```
CONFIG_IDF_TARGET="esp32p4"
CONFIG_ESPTOOLPY_FLASHSIZE_16MB=y
CONFIG_ESPTOOLPY_FLASHSIZE="16MB"
```
If you already have a `sdkconfig` from a previous build, delete it and rebuild so it regenerates from defaults:
```bash
rm sdkconfig
idf.py build
```

#### Step 6: Build
```bash
cd ~/Dropbox/scottsoft_sdn/src/riscvml/esp32-p4/hello_p4
idf.py build
```
Expected: ~197 KB binary, 81% partition free.

#### Step 7: Flash
```bash
idf.py -p /dev/ttyACM0 flash
```
Writes bootloader (0x2000), partition table (0x8000), and app (0x10000). Board auto-resets after flashing.

#### Step 8: Monitor
```bash
idf.py -p /dev/ttyACM0 monitor
```
Press `Ctrl+]` to exit the monitor. **Requires an interactive terminal** (won't work from non-TTY environments like CI or tool subshells).

#### Step 9: Verify Output
```
SPI Flash Size : 16MB
cpu freq: 360000000 Hz
Hello from ESP32-P4!
Chip: esp32p4, 2 core(s), revision 100
Flash size: 16MB (external)
Free heap: 601380 bytes
```

#### All-in-One Quick Reference
```bash
# Activate (every terminal)
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh

# Navigate
cd ~/Dropbox/scottsoft_sdn/src/riscvml/esp32-p4/hello_p4

# Build + Flash + Monitor
idf.py build
idf.py -p /dev/ttyACM0 flash
idf.py -p /dev/ttyACM0 monitor    # Ctrl+] to exit
```

#### Common Gotchas

| Problem | Cause | Fix |
|---------|-------|-----|
| `idf.py: command not found` | Didn't source export.sh | `source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh` |
| Flash size mismatch warning | sdkconfig says 2MB, board has 16MB | Set `FLASHSIZE_16MB` in sdkconfig.defaults, delete sdkconfig, rebuild |
| No `/dev/ttyUSB*` found | P4 uses USB-Serial/JTAG (ACM) | Look for `/dev/ttyACM0` instead |
| Permission denied on serial port | User not in dialout group | `sudo usermod -aG dialout $USER` then re-login |
| Monitor won't start | Not an interactive TTY | Run from a real terminal, not a non-interactive subshell |
| Monitor: `device reports readiness to read but returned no data` | Pressed physical RST1 button — resets USB-Serial/JTAG controller, disconnecting USB | Don't press RST1 while monitor is open. Flash already auto-resets the board via RTS pin. If it happens, wait for auto-reconnect or `Ctrl+]` and restart monitor |
| Flash: `Could not exclusively lock port /dev/ttyACM0: Resource temporarily unavailable` | Another process (usually `idf.py monitor`) is holding the serial port open | Close the monitor first (`Ctrl+]`), then flash. Or use `idf.py -p /dev/ttyACM0 flash monitor` which handles sequencing automatically |

### VS Code Development (Recommended)

Using VS Code with the **Espressif IDF extension** provides a much smoother workflow than the command line alone.

#### Extension Setup
1. Install extension: `espressif.esp-idf-extension` from the VS Code marketplace
2. Run `Ctrl+Shift+P` → **ESP-IDF: Open ESP-IDF Installation Manager** → point to existing ESP-IDF at `~/Dropbox/scottsoft_sdn/esp-idf/`
3. Open the project folder in VS Code
4. Run `Ctrl+Shift+P` → **ESP-IDF: Add .vscode Configuration Folder** — generates IntelliSense config, debug config, and workspace settings
5. Run `Ctrl+Shift+P` → **ESP-IDF: Set Espressif Device Target** → select `esp32p4`
6. Select serial port: click port indicator in status bar → `/dev/ttyACM0`

#### Why VS Code Helps

| Feature | CLI Workflow | VS Code + ESP-IDF Extension |
|---------|-------------|----------------------------|
| **Environment activation** | Must `source export.sh` every terminal | Automatic — extension manages the environment |
| **Build** | `idf.py build` | One-click status bar button or `Ctrl+Shift+P` → Build |
| **Flash** | `idf.py -p /dev/ttyACM0 flash` | One-click with port auto-detected |
| **Monitor** | `idf.py monitor` (terminal only) | Integrated terminal with decoded backtraces |
| **Build+Flash+Monitor** | Three separate commands | Single "Build, Flash and Monitor" action |
| **menuconfig** | Terminal-based curses UI | Searchable GUI via `Ctrl+E G` |
| **IntelliSense** | None | Full code completion, go-to-definition, error squiggles via `compile_commands.json` |
| **Debugging** | Manual OpenOCD + GDB setup | Press `F5` — breakpoints, stepping, watch variables, call stack |
| **Serial port** | Must know `/dev/ttyACM0` | Auto-detected, selectable from status bar |

#### Key Commands
| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+P` → ESP-IDF: Build | Build project |
| `Ctrl+Shift+P` → ESP-IDF: Flash | Flash to board |
| `Ctrl+Shift+P` → ESP-IDF: Monitor | Open serial monitor |
| `Ctrl+E G` | Open menuconfig GUI |
| `F5` | Start OpenOCD debug session |

#### ESP32-P4 Specific Notes
- **OpenOCD board config:** `board/esp32p4-builtin.cfg` (built-in USB-JTAG — no external adapter needed)
- **Toolchain:** Extension auto-selects `riscv32-esp-elf-gcc` when target is `esp32p4`
- **Debugging:** Works over the same USB cable used for flashing — the P4's USB-Serial/JTAG controller supports both simultaneously

## RISCVML Curriculum Relevance

### Direct Teaching Value
- **RISC-V ISA extensibility:** The Xai and Xhwlp custom extensions demonstrate how RISC-V's modular ISA works in practice vs. ARM's fixed architecture
- **Big-little architecture:** Real power management patterns directly applicable to solar/battery embedded systems
- **System-level design:** Companion chip pattern (P4 host + C6 radio) teaches realistic embedded architecture
- **Performance tier:** 400 MHz + 768KB SRAM + 32MB PSRAM enables real ML inference, LVGL GUIs, and camera pipelines not feasible on C3/C6

### Potential Advanced Modules
- Edge vision / camera pipelines with hardware ISP + H.264
- HMI development with MIPI-DSI displays and LVGL
- Heterogeneous multi-chip systems (P4 + C6 communication)
- Custom RISC-V instruction profiling (Xai benchmarking)
- Low-power system design with HP/LP core coordination

### Comparison to C3/C6 (Current RISCVML Targets)

| Feature | ESP32-C3 | ESP32-C6 | ESP32-P4 |
|---------|----------|----------|----------|
| RISC-V Cores | 1 (160 MHz) | 1 HP + 1 LP | 2 HP + 1 LP |
| Max Clock | 160 MHz | 160 MHz | 400 MHz |
| On-chip SRAM | 400 KB | 512 KB | 768 KB |
| PSRAM | Up to 4 MB | Up to 4 MB | Up to 32 MB |
| Wi-Fi | Yes (4) | Yes (6) | No (companion) |
| BLE | 5.0 | 5.0 | No (companion) |
| USB | 1.1 | 1.1 | 2.0 OTG HS |
| Display I/F | SPI/Parallel | SPI/Parallel | MIPI-DSI |
| Camera I/F | No | No | MIPI-CSI + ISP |
| H.264 | No | No | Yes (1080p@30) |
| GPIOs | 22 | 30 | 55 |
| AI Extensions | No | No | Yes (Xai) |
| FPU | No | No | Yes (SP) |
