# RISCVML Submission — RISC-V Summit Europe 2026

**Title:** RISCVML: Teaching RISC-V Embedded ML with Rust — From ESP32-C3 to ESP32-P4

**Author:** Scottie von Bruchhausen
**Affiliation:** RISCVML — riscvml.org — scottie@riscvml.org

**Submission option:** Non-blind

**Topic:** RISC-V related educational activities

**Also consider for talk:** Yes (check the box)

---

**Abstract (250 words):**

RISC-V deployment in embedded systems, IoT, and edge AI has outpaced developer education: most tutorials target C/C++ and cover only basic microcontroller tasks, leaving a gap for building ML systems with modern toolchains. RISCVML addresses this with a Rust curriculum spanning 172 chapters across seven modules, from beginner hardware to on-device ML inference.

The curriculum uses Espressif RISC-V SoCs: the ESP32-C3 (BLE 5.0, ~€3) and ESP32-C6 (Wi-Fi 6, Thread/Matter, ~€4) introduce Rust fundamentals — GPIO, sensors, power management, and wireless protocols. The ESP32-P4 (dual core 400 MHz, AI extensions, 128 bit vector ISA, ~€25) anchors an advanced module: ISP camera pipeline, hardware accelerated 2D rendering, H.264 encoding, DMA orchestration, and vector accelerated ML inference.

These converge in a capstone: a bird detection pipeline capturing MIPI-CSI frames, running quantized detection through esp-dl, driving pan/tilt servos, and recording H.264 video — all in async Rust with ESP-IDF drivers via FFI.

By pairing Rust memory safety with production toolchains (esp-hal, esp-idf-hal) on affordable hardware, and using a mascot to make complex terminology approachable for younger learners, RISCVML lowers the barrier for the next generation of RISC-V developers — supporting Europe's push for sovereign silicon literacy.

---

**Short description for more context (250 words):**

The RISC-V ecosystem faces an asymmetric growth challenge: hardware availability has scaled rapidly — over 20 billion cores shipped — but developer education has not kept pace. Industry surveys identify software as the primary adoption barrier. RISCVML addresses this with four differentiators:

Rust first approach. While nearly all RISC-V tutorials rely on C/C++, RISCVML uses Rust throughout. Compile time memory safety eliminates entire bug classes common in embedded C, and the ecosystem (esp-hal, embassy) has matured to make this viable for production.

Commercially available hardware. The ESP32-C3 (~€3), ESP32-C6 (~€4), and ESP32-P4 (~€25) ensure accessibility across the performance spectrum — learners progress from low cost modules to a 400 MHz dual core with AI extensions within a unified Espressif RISC-V ecosystem.

End to end pipeline. From bare metal firmware to Tauri desktop apps, and from minimal IoT nodes to multimedia edge computing with MIPI displays and cameras, the curriculum spans the full embedded spectrum.

Character driven engagement. The platform's mascot, Count Rusty von Risc-V, embodies the technology stack — making complex terminology approachable for younger learners and career changers entering the ecosystem.

RISCVML also serves as implicit ecosystem testing: exercises against Espressif's esp-rs toolchains generate upstream bug reports. Primary audiences are embedded engineers evaluating ARM to RISC-V transition, university educators seeking structured coursework, IoT hobbyists, younger learners drawn by the character driven approach, and ecosystem companies interested in educational partnerships.
