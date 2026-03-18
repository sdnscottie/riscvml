# svbprj.md — RISCVML Monorepo

This file provides guidance to CC (claude.ai/code) when working with code in this repository.

## Project Overview

**RISCVML** is a Rust-first educational platform for RISC-V embedded systems and ML on Espressif hardware (ESP32-C3, C6, P4). It covers 172 chapters across 7 modules, progressing from bare-metal basics to on-device ML inference.

- **Author:** Scottie von Bruchhausen (scottie@riscvml.org)
- **Affiliation:** RISCVML — riscvml.org
- **GitHub:** github.com/sdnscottie/riscvml
- **Location:** Bad Schwalbach/Wiesbaden, Germany

## Repository Structure

```
riscvml/                              ← you are here
├── svbprj.md                         ← this file (repo-wide guidance)
├── esp32-p4__knuggets/               ← ESP32-P4 knowledge nuggets (has its own svbprj.md)
│   ├── esp32-p4__firststeps/         ← ESP-IDF C project (CMake, idf.py)
│   └── esp32-p4__mlx90640/           ← Rust thermal camera stub (Cargo)
├── riscvml_projects/                 ← curriculum exercise projects
│   └── rs_riscvml_embed__solar_cntrl__freefull/  ← solar controller (has its own svbprj.md)
│       ├── rs_riscvml_embed__solar_cntrl__free/  ← student scaffolding (public)
│       └── rs_riscvml_embed__solar_cntrl__full/  ← reference solution (private)
└── submission_europe_2026/           ← RISC-V Summit Europe 2026 submission (has its own svbprj.md)
    └── files/                        ← docs, diagrams, docx generator
```

## Subproject svbprj.md Files

Each subproject has its own `svbprj.md` with build commands, architecture details, and conventions:

| Subproject | svbprj.md | Summary |
|------------|-----------|---------|
| ESP32-P4 Knuggets | `esp32-p4__knuggets/svbprj.md` | P4 hardware nuggets, ESP-IDF + Rust builds, Olimex board setup |
| Solar Controller | `riscvml_projects/rs_riscvml_embed__solar_cntrl__freefull/svbprj.md` | Free/full exercise pair, 12-panel solar tilt controller |
| Summit Submission | `submission_europe_2026/svbprj.md` | Conference submission, curriculum overview, capstone details |

## Naming Conventions

- **Double underscore `__`**: directory hierarchy separator (e.g., `esp32-p4__knuggets`)
- **Single underscore `_`**: word separator within Cargo package names
- **Knugget docs**: `docs_about__<knugget-name>/` companion directories

## Hardware Targets

| SoC | Role | Price | Rust Target |
|-----|------|-------|-------------|
| ESP32-C3 | Entry-level, BLE 5.0 | ~€3 | `riscv32imc-unknown-none-elf` |
| ESP32-C6 | Wi-Fi 6, Thread/Matter | ~€4 | `riscv32imac-unknown-none-elf` |
| ESP32-P4 | Dual-core 400 MHz, AI extensions, MIPI, H.264 | ~€25 | `riscv32imafc-unknown-none-elf` |

## Common Build Commands

```bash
# Rust projects (cd into crate directory first)
cargo build
cargo test
cargo clippy
cargo fmt -- --check

# ESP-IDF projects (activate toolchain first)
source ~/Dropbox/scottsoft_sdn/esp-idf/export.sh
idf.py build
idf.py -p /dev/ttyACM0 flash monitor   # Ctrl+] to exit monitor
```

## Commit Convention

```
Co-Contributed-By: CC Opus 4.6 <noreply@anthropic.com>
```

## Diagrams

- Draw.io (`.drawio`) files throughout — PNGs excluded via `.gitignore`
- Regenerate: `drawio --export --format png --scale 2 --output X.png X.drawio`
- Mascot SVG: `submission_europe_2026/files/diagrams/riscvml-mascot-rusty.svg`

## Key Technical Patterns

- **Rust-first** with ESP-IDF C drivers via FFI where hardware requires it
- **SQLite3** (`riscvml_detect.db`) for detection logging and telemetry
- **Detect → Visualize → React**: reusable ML pipeline pattern (sensor → classification → SQLite lookup → RGB LED → reaction)
- **esp-hal** for bare-metal, **esp-idf-sys** for ESP-IDF FFI — choose based on feature needs
- **Embassy async** runtime for concurrent embedded tasks
