# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**rs_solar_cntrl** — A Rust solar controller for an ESP32-P4 (RISC-V) that manages a 12-panel TrinaSolar TSM-620NEG19RC.20 array on a wintergarden/greenhouse roof with actuator-driven variable tilt. Part of the RISCVML educational curriculum.

## Repository Layout: Free/Full Pattern

This directory contains two sibling Rust crates that form a **free/full exercise pair**:

- `rs_solar_cntrl__free/` — **Student exercise** (scaffolded, incomplete). Students fill in the implementation.
- `rs_solar_cntrl__full/` — **Reference solution** (gold/complete). Contains the intended implementation, diagrams, and detailed project notes in `svbprj.md`.

When developing: implement features in `__full` first, then create the corresponding scaffolded version in `__free` with TODO markers or stubs for students to complete.

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
```

## Physical Setup

- **Location:** Bad Schwalbach, Germany (~50.1°N, 8.1°E), south-facing garden
- **Panels:** 12x TrinaSolar TSM-620NEG19RC.20 (620 Wp each, bifacial n-Type i-TOPCon)
- **Layout:** 4 across x 3 rows, landscape (~9.53m x 3.40m), total 7.44 kWp
- **Electrical:** 3 strings x 4 panels in series (String Vmp: 165.6 V, String Imp: 14.99 A)
- **Tilt range:** 8deg (greenhouse incline) to 24deg (house roof incline), actuator-controlled

## Intended Architecture

The ESP32-P4 controller will:
1. Read inclinometer (I2C) + calculate optimal sun angle from RTC + GPS coords (50.1°N, 8.1°E)
2. Drive a linear actuator (12/24V DC) via PWM/GPIO to adjust roof tilt (8deg-24deg)
3. Monitor solar panel voltage/current via ADC
4. Log telemetry to SQLite (following RISCVML patterns — see `riscvml_detect.db` in other modules)

## Key Files

- `rs_solar_cntrl__full/svbprj.md` — Detailed project notes and architecture (the authoritative reference)
- `rs_solar_cntrl__full/docs_about__rs_solar_cntrl/` — Draw.io diagrams (panel layout, tilt mechanism)

## Conventions

- Commits use: `Co-Contributed-By: Claude Opus 4.6 <noreply@anthropic.com>`
- Follow RISCVML patterns: SQLite for telemetry, embedded-hal traits for hardware abstraction
- Prefer `esp-hal` (bare-metal) or `esp-idf-sys` (ESP-IDF FFI) depending on hardware feature needs
- Diagrams go in `docs_about__rs_solar_cntrl/` directory
- Regenerate diagram PNGs: `drawio --export --format png --scale 2 --output X.png X.drawio`
