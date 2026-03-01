# svbprj.md — rs_riscvml_embed_solar_cntrl (Free Edition)

## Project Overview

**rs_riscvml_embed_solar_cntrl** — A Rust solar controller for an ESP32-P4 (RISC-V) that manages a 12-panel TrinaSolar TSM-620NEG19RC.20 array on a wintergarden/greenhouse roof with actuator-driven variable tilt. Part of the RISCVML educational curriculum.

This is the **free scaffolded edition**. The architecture, types, and hardware abstraction are provided — your task is to implement the core logic in the TODO stubs.

For the **full reference solution** with complete implementations, detailed wiring diagrams, and expert guidance, visit: **[Agrarobotics.com/training](https://Agrarobotics.com/training)**

## Physical Setup

- **Location:** Bad Schwalbach, Germany (~50.1°N, 8.1°E), south-facing garden
- **Panels:** 12× TrinaSolar TSM-620NEG19RC.20 (620 Wp each, bifacial n-Type i-TOPCon)
- **Layout:** 4 across × 3 rows, landscape orientation (~9.53m × 3.40m)
- **Total rated power:** 7,440 Wp (7.44 kWp)
- **Electrical config:** 3 strings × 4 panels in series (String Vmp: 165.6 V, String Imp: 14.99 A)
- **Tilt range:** 8° (greenhouse natural incline) to 24° (house roof incline), actuator-controlled

## Build Commands

```bash
cargo build          # Build (host target)
cargo test           # Run tests — your progress tracker!
cargo clippy         # Lint
cargo fmt --check    # Check formatting
```

## Your Challenge

Implement the TODO stubs in these modules (run `cargo test` to check progress):

### Level 1 — Guided (hints provided)
- **sun_position.rs** — `optimal_tilt()`, `seasonal_default()`
- **inclinometer.rs** — `read_angle()`, `raw_to_angle()`
- **adc_monitor.rs** — `read_all_strings()`, voltage/current conversions

### Level 2 — Independent (minimal hints)
- **actuator.rs** — `move_to_angle()`, `proportional_duty()`
- **safety.rs** — `poll()`
- **scheduler.rs** — `tick()`, `calculate_target()`
- **telemetry_db.rs** — all INSERT/UPSERT methods

### Given Complete (study these for patterns)
- **config.rs** — all system constants
- **types.rs** — shared types and error handling
- **ffi.rs** — safe FFI wrappers (with ESP-IDF documentation)
- **hal.rs** — hardware peripheral initialization
- **main.rs** — demo orchestrator

## Project Structure

- `src/` — Rust source files (your workspace)
- `docs_about__rs_riscvml_embed__solar_cntrl__free/` — simplified diagrams (see full version for detailed wiring)

## Conventions

- Commits use: `Co-Contributed-By: Claude Opus 4.6 <noreply@anthropic.com>`
- Follow RISCVML patterns: SQLite for telemetry, embedded-hal traits for hardware
- Diagrams go in `docs_about__rs_riscvml_embed__solar_cntrl__free/` directory
