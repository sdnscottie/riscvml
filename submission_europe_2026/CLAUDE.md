# RISCVML — RISC-V Summit Europe 2026 Submission

## Project Overview

Conference submission for RISC-V Summit Europe 2026 in Bologna, Italy (June 8–12, 2026). The submission presents RISCVML, a comprehensive Rust-based educational platform for RISC-V embedded systems development on Espressif hardware.

**Submission portal:** https://cfp.riscv-europe.org/eu-summit-2026/cfp
**Submission deadline:** March 2, 2026 (AOE — Anywhere on Earth)
**Notification:** April 17–24, 2026
**Final materials deadline:** May 29, 2026
**Conference:** June 8–12, 2026, Palazzo dei Congressi, Bologna
**Contact for questions:** europe-program-committee@riscv.org

## Author

- **Name:** Scottie von Bruchhausen
- **Affiliation:** RISCVML — riscvml.org
- **Email:** scottie@riscvml.org
- **Location:** Bad Schwalbach/Wiesbaden, Germany (originally from Biloxi, Mississippi)

## Submission Details

- **Type:** Non-blind submission
- **Topic:** RISC-V related educational activities
- **Also request talk consideration:** Yes (check the box — poster is default)
- **Title:** RISCVML: Teaching RISC-V Embedded ML with Rust — From ESP32-C3 to ESP32-P4
- **Abstract:** 206 portal-words (max 250) — see `RISCVML-Abstract-250-words.md`
- **Short description:** 237 portal-words (max 250) — see `RISCVML-Abstract-250-words.md`
- **Author bio:** 148 words (max 150) — see `RISCVML-Abstract-250-words.md`
- **Extended abstract PDF:** 2 pages, A4 — see `RISCVML-Summit-Europe-2026-Submission.pdf`
- **Templates:** https://github.com/riscv-europe/riscv-europe-summit-templates

### Non-blind submission structure (as required)
1. Short summary of the contribution
2. Why this is important for the community/industry adoption
3. How this work contributes to enable/foster the ecosystem
4. Target audience

## Key Technical Content

### Hardware Coverage (progressive ladder)
| SoC | Role | Price Point |
|-----|------|-------------|
| ESP32-C3 | Entry-level, single-core 160 MHz RISC-V, BLE 5.0 | ~€3 |
| ESP32-C6 | Wi-Fi 6, BLE 5.3, Zigbee/Thread, Matter-ready | ~€4 |
| TTGO T-Beam | LoRa long-range, GPS, ESP32-based | ~€30 |
| ESP32-P4 | Dual-core 400 MHz RISC-V, AI extensions, 128-bit vector, MIPI-CSI/DSI, H.264, PPA, no built-in radio | ~€25 dev board |

### Curriculum: 172 chapters, 7 modules
- **Modules 1–5:** Rust fundamentals, GPIO/sensors/PCA9685, power/solar, LoRa on T-Beam, multi-device ESP-NOW/MQTT
- **Module 6:** ESP32-P4 high-performance — dedicated to P4 hardware subsystems via Rust
- **Module 7:** Firmware-to-desktop bridge via Tauri (Rust + WebView)

### ESP32-P4 Hardware Showcases (Module 6)
Each P4 capability is addressed with Rust:

| P4 Feature | Rust Approach |
|------------|---------------|
| ISP (Image Signal Processor) | Type-safe register abstractions for MIPI-CSI camera pipeline config |
| 2D GPU / PPA (Pixel Processing Accelerator) | Hardware-accelerated rendering, LVGL Rust bindings |
| H.264 Video Pipeline | Zero-copy frame management via Rust ownership model |
| DMA / 2D-DMA | Async embassy orchestration, borrow checker on DMA descriptors |
| AI/Vector (128-bit SIMD) | Inline assembly + intrinsic wrappers around RISC-V vector ISA |

### Capstone Project: Bird Detection Pipeline on ESP32-P4
Real-world end-to-end system exercising every P4 subsystem. Rust-first architecture with ESP-IDF drivers (esp-video, esp-detection/esp-dl) via FFI.

**Phase 1 — Camera → ISP → Display Preview (30–60 FPS)**
- MIPI-CSI capture via Camera Controller Driver
- ISP pipeline: white balance, auto-exposure, demosaicing
- DMA-driven frame transfer to MIPI-DSI display
- PPA for scaling/rotation
- Embassy async runtime, zero-copy buffer flow

**Phase 2 — On-Device Bird Detection Inference**
- esp-detection / esp-dl for quantized object detection
- PPA hardware-downscales frames to model input resolution
- Bounding box overlay via PPA alpha blending
- 128-bit vector extensions accelerate esp-dl tensor ops
- Detection results (species, confidence, bounding box, timestamp) logged to `riscvml_detect.db` (SQLite3)
- RGB LED lit per detected species — color mapping stored in `bird_led_colors` table in `riscvml_detect.db`

**Phase 3 — Tracking, Servos & Recording**
- PCA9685 pan/tilt servo control (reuses Module 2 abstractions)
- H.264 hardware encoder: 1080p@30fps, zero-copy frame ingestion
- Companion ESP32-C6 via ESP-Hosted/SDIO for Wi-Fi 6
- MQTT alerts + RTSP streaming
- Demonstrates P4's intended companion-chip architecture

### ML Detect → Visualize → React Pattern
The bird detection capstone is one instance of a reusable embedded ML architecture. The general pattern:

```
Sensor Input → ML Classification → SQLite Lookup (class → RGB) → RGB LED → Reaction
```

Each use case swaps the sensor, model, color map table, and reaction while sharing the same ESP32-P4 infrastructure (esp-dl, 128-bit vector ISA, SQLite3, Rust + Embassy async, GPIO PWM → RGB LED).

| Use Case | Sensor | ML Model | Color Map Table | LED Meaning | Reaction |
|----------|--------|----------|-----------------|-------------|----------|
| **Bird Detection** (capstone) | MIPI-CSI camera | Bird species detection | `bird_led_colors` | Species color | Servo track, H.264 record, MQTT alert |
| **Object / Obstacle** | Camera / LiDAR | Object classification | `object_led_colors` | Threat level | Proximity alert, buzzer, event log |
| **Plant Health** | Camera / Multispectral | Health assessment | `health_led_colors` | Health status | Irrigation valve, fertilizer dose, dashboard |
| **Sound Classification** | I2S/PDM microphone | Sound/voice classification | `sound_led_colors` | Event type | Security alert, audio record, push notification |

All color map tables live in `riscvml_detect.db` with schema: `(class_name TEXT, r INTEGER, g INTEGER, b INTEGER)`.

### ESP-IDF Components Referenced
- Camera Controller Driver (CSI / ISP DVP / LCD_CAM DVP)
- ISP (camera → processed frames via DMA)
- PPA (rotate/scale/mirror/blend)
- LCD stack including MIPI-DSI
- esp-detection / esp-dl for object detection models
- esp-video framework for camera applications

## Branding Decisions

- **Primary domain:** riscvml.org (for conference/academic contexts)
- **Website:** riscvml.com
- **Email:** scottie@riscvml.org (Google Workspace on riscvml.org)
- **Redirect:** riscml.org → 301 redirect to riscvml.org (owned, weaker brand)
- **Additional email aliases to set up:** info@riscvml.org, support@riscvml.org

## Conference Strategy

### Immediate (by March 2, 2026)
- [ ] Submit at cfp.riscv-europe.org/eu-summit-2026/cfp
- [ ] Register on platform, fill author info
- [ ] Paste 232-word abstract into form
- [ ] Select non-blind, check "consider for talk" box
- [ ] Upload PDF (max 10 MB)

### If Accepted (by May 29, 2026)
- [ ] Upload final poster/presentation (max 20 MB)
- [ ] Print A0 portrait poster — include QR codes to riscvml.org
- [ ] Register for core conference (Tue Jun 9 – Thu Jun 11)
- [ ] Prepare live demo: bird detection pipeline on ESP32-P4 hardware
- [ ] Book travel/accommodation in Bologna

### Other European Conferences to Target
| Event | Date | Location | Status |
|-------|------|----------|--------|
| Embedded World 2026 | Mar 10–12 | Nuremberg | Attend as visitor, network at RISC-V Pavilion Hall 5 |
| Elektor RISC-V Online Conference | Apr 15 | Online | Consider attending (€199 early bird) |
| RISC-V Summit Europe 2026 | Jun 8–12 | Bologna | **SUBMIT BY MAR 2** |
| FOSDEM 2027 | Jan/Feb 2027 | Brussels | Target for next year |

## File Inventory

| File | Purpose |
|------|---------|
| `RISCVML-Summit-Europe-2026-Submission.pdf` | Upload to submission portal |
| `RISCVML-Summit-Europe-2026-Submission.docx` | Editable source document |
| `RISCVML-Abstract-250-words.md` | Abstract text + form field instructions |
| `create_submission.js` | Node.js script to regenerate docx (uses `docx` npm package) |
| `CLAUDE.md` | This file — project context |
| `claude_extra.md` | Extended context: landscape analysis, Idea Box, storage/HPC, embedded-smart ML thesis, Rusty mascot |

### Diagrams (`files/diagrams/`)

| File | Purpose |
|------|---------|
| `hardware-ladder.drawio` | ESP32-C3 → C6 → P4 hardware progression with specs, prices, curriculum topics |
| `bird-detection-pipeline.drawio` | 3-phase capstone pipeline: Capture → ML Inference → Track & Record, with SQLite + RGB LED. Color-coded: red = Rust-owned, gray = ESP-IDF via FFI |
| `rust-embedded-ml-stack.drawio` | 5-layer architecture: Application → Rust HAL → FFI boundary → ESP-IDF C drivers → Hardware, with safety annotations |
| `ml-detect-react-pattern.drawio` | Reusable ML pattern: Sensor → Classification → SQLite color lookup → RGB LED → Reaction, with 4 concrete use cases (bird, object, plant, sound) |

Export diagrams as SVG from draw.io for inclusion in the submission PDF.

## Regenerating the Document

```bash
# Requires: npm install -g docx
node create_submission.js
# Validate
python3 scripts/office/validate.py RISCVML-Summit-Europe-2026-Submission.docx
# Convert to PDF (requires LibreOffice)
libreoffice --headless --convert-to pdf RISCVML-Summit-Europe-2026-Submission.docx
```

## Monetization Context

- Individual chapters: €2 each
- Complete bundle: €30
- Free introductory chapters for trust-building
- Long-term strategy: potential acquisition by Espressif, SiFive, or similar RISC-V ecosystem company seeking developer education content

## Key Arguments for Reviewers

1. **Ecosystem gap:** 20B+ RISC-V cores deployed, but developer education hasn't kept pace
2. **Rust-first:** Nearly all existing RISC-V tutorials use C/C++ — Rust's safety guarantees are uniquely valuable for embedded
3. **Full hardware spectrum:** €3 C3 to 400 MHz P4 with AI extensions — all within one curriculum
4. **Concrete capstone:** Bird detection pipeline is demonstrable, tangible, exercises every P4 subsystem
5. **Pragmatic interop:** Rust-first but uses ESP-IDF via FFI where needed — not dogmatic
6. **European relevance:** Aligns with EU Chips Act, digital sovereignty, open-standard advocacy

## Bologna Strategic Positioning

### Summit Landscape — Key Themes
1. **RISC-V Adoption Accelerating** — from embedded → AI → data centers → automotive → HPC; Europe hosts ~⅓ of global RISC-V community
2. **AI & Vector Computing Breakthroughs** — matrix/vector extensions for AI workloads, edge AI demos, real-time imaging accelerators
3. **High-Performance & Data Center RISC-V** — expanding beyond MCUs into cloud & HPC workloads
4. **Security, Sovereignty & European Chips Strategy** — RISC-V as strategic independence from proprietary architectures; trusted architectures as key research
5. **Space & Safety-Critical Systems** — ESA developing RISC-V SoCs for upcoming missions
6. **Ecosystem & Open Innovation** — emphasis on collaboration across academia, industry, and government

### Where RISCVML Lands
- **Theme 2 is the sweet spot** — bird-detection capstone on ESP32-P4 lands in the active AI/edge conversation
- **Theme 1 strengthens curriculum value** — as RISC-V goes mainstream, the talent pipeline becomes a bottleneck; RISCVML fills workforce development gap
- **Theme 4 is the European differentiation angle** — ESP32-P4 (Espressif), open toolchains, Rust memory safety → speaks to sovereignty/security priorities
- **Theme 6** — watch for partnership opportunities with hardware partners or academic groups at Bologna

### Idea Box Initiative — Conference Engagement Strategy
**Core framing:** Show up with a working solution that's also an invitation to collaborate. Not "someone should make Rust ML easier on RISC-V" but "here's the platform, what would you build on top of it?"

- **Interactive challenge:** "Build your first Rust ML pipeline on RISC-V in 30 minutes" — invite attendees to propose new capstone projects (industrial vibration monitoring, agricultural sensor fusion, acoustic anomaly detection, etc.)
- **Call to action:** One-page handout / QR code → riscvml.org: "Submit your edge ML challenge — best ideas get built into the curriculum"
- **Positioning:** Ecosystem builder, not just course creator. Propose a "RISC-V Developer Experience Roadmap" session.

### High-Value Idea Box Topics
| Category | Ideas |
|----------|-------|
| Developer Experience | Unified debugger/profiling stack, vector extension visualization, Rust-first embedded SDKs |
| Edge AI & Vector | Standardized AI runtime for RVV, TinyML optimization pipelines, open edge AI benchmarks |
| Security & Trust | Reference secure boot, open TEE profiles, side-channel resistance libraries |
| Interoperability | Portable HAL across RISC-V SoCs, standard camera/ISP interfaces, unified DMA abstraction |

## RISE Partnership Strategy

### What is RISE?
[RISE (RISC-V Software Ecosystem)](https://riseproject.dev/) is a Linux Foundation Europe project accelerating commercial-ready open-source software for RISC-V. Governing board: Google, Intel, NVIDIA, Qualcomm, Samsung, SiFive, Red Hat, Andes, Imagination, MediaTek, Rivos, T-Head, Ventana.

### Why RISE + RISCVML
1. **RISE has infrastructure, RISCVML has education** — RISE builds toolchains, language runtimes, Linux distros; RISCVML teaches developers to use them. RISE builds the road; RISCVML teaches people to drive.
2. **Developer talent bottleneck** — RISE members contribute engineering talent but the pipeline that *creates* RISC-V developers barely exists outside China's SOPIC. RISCVML is the Western/European answer.
3. **Rust rising to Tier-1** — `riscv64gc-unknown-linux-gnu` is Tier-2 with proposal for Tier-1. RISE cares about language runtimes; RISCVML is the ready-made Rust training pipeline.
4. **European governance alignment** — RISE hosted by LF Europe, RISCVML based in Munich. Both benefit from EU Chips Act / digital sovereignty narrative.
5. **MCU-to-application-processor bridge** — RISE targets application-class RISC-V (RVA23). RISCVML's curriculum bridges from bare-metal MCUs (C3/C6/P4) upward — the on-ramp to RISE's ecosystem.

### Concrete Collaboration Opportunities
| Opportunity | How It Works |
|-------------|-------------|
| RISE-endorsed curriculum | RISCVML becomes recommended training for RISE's developer community |
| Hardware seeding | RISE members (SiFive, Espressif) seed dev boards to RISCVML learners |
| Upstream bug reports | RISCVML exercises generate real-world testing of esp-rs, esp-hal, RISE-supported toolchains |
| Conference presence | Joint booth/demo at RISC-V Summit Europe — RISE infrastructure + RISCVML education |
| Developer Appreciation Program | RISCVML students contribute porting/testing work qualifying for RISE recognition |

### The Pitch
> RISE is building the RISC-V software ecosystem; RISCVML is building the developers who will use it.

## Yocto Project — Linux on RISC-V

### Recent Development
In mid-2025, [RISC-V International and RISE partnered to become Platinum Members of the Yocto Project](https://riscv.org/riscv-news/2025/05/risc-v-international-and-the-rise-project-join-forces-for-yocto-project-support/). This formalizes RISC-V support across all profiles (including RVA23) with 4-year LTS cycles, SBOM generation, and build reproducibility. **RISE + Yocto + RISC-V Linux is the blessed stack for production embedded Linux on RISC-V.**

### Extended Hardware Ladder (MCU → Linux → HPC)

| Tier | Hardware | OS | Price | RISCVML Role |
|------|----------|-----|-------|-------------|
| Entry | ESP32-C3 | no_std / bare-metal | ~€3 | Modules 1–3 |
| Mid | ESP32-C6 / P4 | no_std + ESP-IDF via FFI | €4–25 | Modules 4–6, capstone |
| **Linux** | **VisionFive 2 Lite** | **Yocto / Ubuntu** | **~€20–65** | **New Module 8** |
| HPC | Multi-node cluster | Linux + distributed Rust | varies | Future |

Note: ESP32-P4 is a powerful MCU but does NOT run Linux (no MMU). The VisionFive 2 Lite (StarFive JH7110S, quad-core 64-bit, ~€20 base) is the natural step up — runs Ubuntu 24.04, NVMe support, classroom-affordable.

### What a Yocto/Linux Module Would Teach
1. Custom Linux image with Yocto — `meta-riscv` BSP layer, minimal image for VisionFive 2
2. Rust on Linux RISC-V — `riscv64gc-unknown-linux-gnu` target
3. Yocto + Rust integration — `meta-rust-bin` layer, cross-compilation, recipe writing
4. ML inference on Linux RISC-V — same patterns as P4 but with full std library, filesystem, networking
5. MCU-to-Linux bridge — ESP32-P4 (no_std) talks to VisionFive 2 (std) via MQTT/UART — same Rust, different targets
6. EU Cyber Resilience Act compliance — Yocto's SBOM generation directly relevant to CRA preparedness

### Full Curriculum Stack
```
RISCVML Curriculum (Rust-first, all RISC-V)
├── Bare-metal MCU:  ESP32-C3/C6    (no_std, esp-hal)
├── Advanced MCU:    ESP32-P4        (no_std + FFI, esp-dl, capstone)
├── Embedded Linux:  VisionFive 2    (Yocto, std Rust, meta-riscv)
└── Distributed:     Multi-node      (cluster inference, HPC glue)
```

## Storage & HPC — Extended RISC-V ML Context

### Storage-Bound ML
- ML is often storage-bound, not compute-bound: bottlenecks in NVMe reads, data parsing, tensor movement
- Rust advantage: async I/O (Tokio), zero-copy parsing, tight memory layout/buffer control
- Computational storage / "compute near data": push inference closer to SSD/controller on RISC-V cores
- Applications: dedup classification, anomaly detection on logs, content tagging, pre-ranking near storage

### HPC
- HPC loves vector + manycore → RISC-V's trajectory (vectorization, high core counts, efficient math kernels)
- Rust as HPC "glue code": job orchestration, distributed data loading, checkpointing, telemetry, correctness under concurrency
- Distributed ML patterns: MPI-style batch inference, parameter-server/all-reduce, pipeline parallel, vector DB search

### Strategic Takeaways
- **Storage-bound insight is strongest differentiator** — most ML education focuses on compute; teaching memory layout, zero-copy parsing, memory-mapped weight loading gives immediately valuable production skills
- **"Compute near data" maps to existing capstone** — bird detection on ESP32-P4 is already this pattern at edge scale
- **Curriculum gap to address:** explicit middle tier between ESP32-P4 embedded and HPC distributed (e.g., multi-node Rust inference on VisionFive 2 cluster)

## Embedded-Smart ML Architecture — Technical Thesis

### Core Principle
Embedded RISC-V + Rust is the right place to rethink data representation, memory movement, sparsity, and precision because embedded constraints force correct design.

### Four Pillars

**1. Data Representation — Make the Model Fit the Device**
- Store weights/activations in execution format, not FP32
- Rust: explicit tensor layouts (`struct Tensor<T, Layout>`), `#[repr(C)]` fixed-size arrays
- Formats: INT8/INT4 weights, per-channel scales, block-quant, bitpacked sparsity masks

**2. Memory Movement — Treat Copies as the Enemy**
- Streaming inference pipeline: read → preprocess → layer compute → next layer
- Arena allocator with lifetimes, `&mut [u8]` scratch buffers, no-aliasing enforcement
- Techniques: in-place ops, operator fusion (Conv+Bias+ReLU), tiling for SRAM/cache, DMA-friendly layouts

**3. Sparsity — Skip Work and Skip Reads**
- Block sparsity (4×1, 8×1) for simple inner loops
- Packed nonzeros + compact index/mask; apply on 1×1 conv, FC, embedding tables
- Rust: bitmask iteration without UB, bounds-checked indexing (dev) → unsafe fast path (prod)

**4. Precision — Mix It Layer-by-Layer**
- Activations: INT8, Accumulators: INT32, Sensitive layers: FP16+, Output logits: INT16/FP16
- `Quantized`/`Requantize` traits, compile-time kernel selection by dtype

### Concrete Architecture
- **Model Packer (offline, PC-side):** trained model → quantize (INT8/INT4) → prune to structured sparsity → single binary blob (headers + weights + scales)
- **Runtime (on RISC-V):** memory-map blob, fixed arena for activations, fused kernels with tiling, optional sparse kernels per layer
- **Profiling Hooks:** count bytes moved, MACs skipped via sparsity, cycles per op

### Pedagogical Sequence
1. Naive inference with `Vec<f32>` — blows memory budget
2. Arena allocation with lifetime-bounded scratch buffers — fraction of memory traffic
3. Operator fusion (Conv+Bias+ReLU single pass) — eliminates intermediate buffers
4. Profiling hooks give hard numbers at each step
5. Students learn Rust memory management because the bird detector won't run in real-time without it

### Bologna Framing Quote
> "RISCVML teaches ML engineering from the constraints up. Instead of training models in Python and optimizing later, students build inference runtimes in Rust on RISC-V where data representation, memory movement, sparsity, and precision are first-class architectural decisions — enforced by the type system, measured by profiling hooks, and motivated by real hardware limits."

## Rusty — RISCVML Mascot

### Character Design
- **Body:** Rust crab (Ferris nod), rusty orange/copper metallic coloring
- **Head:** Purple transparent neural network brain dome with visible nodes and weighted connections (4-layer neural net)
- **Eyes:** Glowing teal with friendly expression
- **Chest:** RISC-V chip die pattern with IC pins, green "V" emblem
- **Shoulder:** Rust gear emblem with "R" stamp
- **Antenna:** Broadcasting tip in RISC-V green with signal waves
- **Nameplate:** Color-coded — RISC (rust orange), V (green), ML (purple)

### Pincher Evolution
- **v1:** Basic crab claws with gear joints → `riscvml-mascot-rusty.svg`
- **v2:** Articulated mechanical claws with LED status lights, hydraulic pistons → `riscvml-mascot-rusty-v2.svg`
- **v3 (current):** Lightsaber-style energy pinchers → `riscvml-mascot-rusty-v3-sabers.svg`
  - Left claw: Green saber (RISC-V) — dual blades, 5-layer glow
  - Right claw: Purple saber (ML) — matching construction, mirrored
  - Darker background (#0D0D1A) to make glow pop

All SVGs are fully vector — scale to any size for favicon, slides, stickers, t-shirts, conference badges.

## Open Threads
- [ ] Detailed chapter outline for Model Packer + Runtime + Profiling Hooks module (8–10 chapters)
- [ ] Conference-ready one-pager synthesizing storage/HPC/embedded-smart themes
- [ ] Further mascot iterations (blade angles, ESP32-P4 chip detail, bird for capstone)
- [ ] "RISC-V Developer Experience Roadmap" session proposal for Bologna
- [ ] Curriculum middle tier: multi-node Rust inference on Linux-capable RISC-V boards
- [ ] RISE outreach — contact RISE TSC about curriculum endorsement / developer program alignment
- [ ] Module 8 outline: Yocto + Rust on VisionFive 2 Lite (meta-riscv, meta-rust-bin, CRA/SBOM)
- [ ] Acquire VisionFive 2 Lite dev board for testing and curriculum development
- [ ] Explore RISE Developer Appreciation Program for RISCVML student contributions
