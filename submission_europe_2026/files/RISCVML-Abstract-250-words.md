# RISCVML Submission — RISC-V Summit Europe 2026

**Title:** RISCVML: Teaching RISC-V Embedded ML with Rust — From ESP32-C3 to ESP32-P4

**Author:** Scottie von Bruchhausen
**Affiliation:** RISCVML — riscvml.org — scottie@riscvml.org

**Submission option:** Non-blind

**Topic:** RISC-V related educational activities

**Also consider for talk:** Yes (check the box)

---

**Abstract (250 words):**

The rapid deployment of RISC-V in embedded systems, IoT, and edge AI has outpaced developer education: most existing tutorials target C/C++ and cover only basic microcontroller tasks, leaving a gap for engineers who need to build machine-learning-capable systems with modern toolchains. RISCVML addresses this gap with a structured, Rust-first curriculum spanning 172 chapters across seven modules, progressing from entry-level hardware to on-device ML inference.

The curriculum uses commercially available Espressif RISC-V SoCs as its teaching platform: the ESP32-C3 (single-core, BLE 5.0, ~€3) and ESP32-C6 (Wi-Fi 6, Thread/Matter, ~€4) introduce embedded Rust fundamentals — GPIO, sensors, power management, and wireless protocols. The ESP32-P4 (dual-core 400 MHz, AI extensions, 128-bit vector ISA, ~€25 dev board) anchors an advanced module covering its ISP camera pipeline, hardware-accelerated 2D rendering, H.264 video encoding, DMA orchestration, and vector-accelerated ML inference.

These subsystems converge in a real-world capstone: an on-device bird-detection pipeline that captures frames via MIPI-CSI, runs quantized object detection through esp-dl, drives pan/tilt servos for tracking, and records H.264 video — all orchestrated in async Rust with ESP-IDF drivers integrated via FFI where hardware support requires it.

By pairing Rust's memory-safety guarantees with production-ready toolchains (esp-hal, esp-idf-hal) on affordable, widely available hardware, RISCVML lowers the barrier for engineers, students, and hobbyists entering the RISC-V ecosystem — directly supporting Europe's push for open-standard, sovereign silicon literacy.
