# svbprj.md

This file provides guidance to CC (claude.ai/code) when working with code in this repository.

## Project Overview

**rs_riscvml__embedded_mini_ups** — A Rust mini UPS controller for an ESP32 (RISC-V) using the Waveshare UPS Power Module (C). Part of the RISCVML educational curriculum.

The Waveshare UPS Power Module (C) provides uninterruptible power via 18650 Li-ion batteries with charge management, voltage step-up, and I2C telemetry (INA219 current/voltage sensor + fuel gauge).

## Repositories

| Repo | Visibility | URL |
|------|-----------|-----|
| **riscvml** (monorepo) | Public | https://github.com/sdnscottie/riscvml |
| **__free** (student exercise) | Public | https://github.com/sdnscottie/rs_riscvml__embedded_mini_ups__free |
| **__full** (reference solution) | Private | https://github.com/sdnscottie/rs_riscvml__embedded_mini_ups__full |

## Repository Layout: Free/Full Pattern

This directory contains two sibling Rust crates that form a **free/full exercise pair**:

- `rs_riscvml__embedded_mini_ups__free/` — **Student exercise** (scaffolded, incomplete). Students fill in the implementation.
- `rs_riscvml__embedded_mini_ups__full/` — **Reference solution** (gold/complete). Contains the intended implementation, diagrams, and detailed project notes.

When developing: implement features in `__full` first, then create the corresponding scaffolded version in `__free` with TODO markers or stubs for students to complete.

## Directory Structure

```
rs_riscvml__embedded_mini_ups/
├── svbprj.md                                          ← this file
├── rs_riscvml__embedded_mini_ups__free/
│   └── docs_about__embedded_mini_ups__free/           ← diagrams, wiring docs (public)
└── rs_riscvml__embedded_mini_ups__full/
    └── docs_about__embedded_mini_ups__full/            ← diagrams, wiring docs (private)
```

## Build Commands

Both crates use Rust edition 2024 with no external dependencies yet.

```bash
# Build/run/test either crate (cd into the crate directory first)
cargo build
cargo run
cargo test
cargo clippy
cargo fmt
cargo fmt -- --check
```

Cross-compilation targets (once toolchain is configured):
```bash
cargo build --target riscv32imc-unknown-none-elf    # ESP32-C3
cargo build --target riscv32imac-unknown-none-elf   # ESP32-C6
cargo build --target riscv32imafc-unknown-none-elf  # ESP32-P4
```

## Hardware: Waveshare UPS Power Module (C)

- **Battery:** 2x 18650 Li-ion cells (not included)
- **Output:** 5V regulated (step-up converter)
- **Monitoring:** I2C — INA219 (bus voltage, shunt current, power) + battery fuel gauge
- **Charging:** USB-C input with pass-through charging
- **Use case:** Keep ESP32 running during power outages, monitor battery health, log charge/discharge cycles

## Intended Architecture

The ESP32 controller will:
1. Read battery voltage, current, and remaining capacity via I2C (INA219 / fuel gauge)
2. Detect power loss (mains disconnected) and trigger UPS mode
3. Monitor charge/discharge cycles and battery health
4. Log telemetry to SQLite (following RISCVML patterns)
5. Drive RGB LED status indicators (green=charged, yellow=discharging, red=low battery)

## Conventions

- Commits use: `Co-Contributed-By: CC Opus 4.6 <noreply@anthropic.com>`
- Follow RISCVML patterns: SQLite for telemetry, embedded-hal traits for hardware abstraction
- Prefer `esp-hal` (bare-metal) or `esp-idf-sys` (ESP-IDF FFI) depending on hardware feature needs
- Diagrams go in `docs_about__embedded_mini_ups__[free|full]/` directories
- Regenerate diagram PNGs: `drawio --export --format png --scale 2 --output X.png X.drawio`
