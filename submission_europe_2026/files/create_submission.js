const fs = require("fs");
const {
  Document, Packer, Paragraph, TextRun, Header, Footer,
  AlignmentType, HeadingLevel, PageNumber, PageBreak,
  Column, SectionType, TabStopType, PositionalTab,
  PositionalTabAlignment, PositionalTabRelativeTo, PositionalTabLeader,
  NumberFormat
} = require("docx");

// ---- Content ----

const title = "RISCVML: Teaching RISC-V Embedded ML with Rust \u2014 From ESP32-C3 to ESP32-P4";

const authors = "Scottie von Bruchhausen";
const affiliation = "RISCVML \u2014 riscvml.org \u2014 scottie@riscvml.org";

const abstractText = `The rapid deployment of RISC-V in embedded systems, IoT, and edge AI has outpaced developer education: most existing tutorials target C/C++ and cover only basic microcontroller tasks, leaving a gap for engineers who need to build machine-learning-capable systems with modern toolchains. RISCVML addresses this gap with a structured, Rust-first curriculum spanning 172 chapters across seven modules, progressing from entry-level hardware to on-device ML inference.\n\nThe curriculum uses commercially available Espressif RISC-V SoCs as its teaching platform: the ESP32-C3 (single-core, BLE 5.0, ~\u20AC3) and ESP32-C6 (Wi-Fi 6, Thread/Matter, ~\u20AC4) introduce embedded Rust fundamentals \u2014 GPIO, sensors, power management, and wireless protocols. The ESP32-P4 (dual-core 400 MHz, AI extensions, 128-bit vector ISA, ~\u20AC25 dev board) anchors an advanced module covering its ISP camera pipeline, hardware-accelerated 2D rendering, H.264 video encoding, DMA orchestration, and vector-accelerated ML inference.\n\nThese subsystems converge in a real-world capstone: an on-device bird-detection pipeline that captures frames via MIPI-CSI, runs quantized object detection through esp-dl, drives pan/tilt servos for tracking, and records H.264 video \u2014 all orchestrated in async Rust with ESP-IDF drivers integrated via FFI where hardware support requires it.\n\nBy pairing Rust\u2019s memory-safety guarantees with production-ready toolchains (esp-hal, esp-idf-hal) on affordable hardware, and using a character-driven mascot to make complex terminology visually approachable for younger learners, RISCVML lowers the barrier for the next generation of RISC-V developers \u2014 supporting Europe\u2019s push for open-standard, sovereign silicon literacy.`;

// ---- Helper functions ----

function makeFooter() {
  return new Footer({
    children: [
      new Paragraph({
        alignment: AlignmentType.LEFT,
        children: [
          new TextRun({
            text: "RISC-V Summit Europe, Bologna, 8-12th June 2026",
            font: "Times New Roman",
            size: 18, // 9pt
            italics: true,
          }),
          new TextRun({
            children: [
              new PositionalTab({
                alignment: PositionalTabAlignment.RIGHT,
                relativeTo: PositionalTabRelativeTo.MARGIN,
                leader: PositionalTabLeader.NONE,
              }),
            ],
            font: "Times New Roman",
            size: 18,
          }),
          new TextRun({
            children: [PageNumber.CURRENT],
            font: "Times New Roman",
            size: 18,
            bold: true,
          }),
        ],
      }),
    ],
  });
}

function heading1(text) {
  return new Paragraph({
    alignment: AlignmentType.CENTER,
    spacing: { before: 160, after: 80 },
    children: [
      new TextRun({
        text: text,
        font: "Times New Roman",
        size: 24, // 12pt
        bold: true,
      }),
    ],
  });
}

function heading2(text) {
  return new Paragraph({
    alignment: AlignmentType.LEFT,
    spacing: { before: 120, after: 60 },
    children: [
      new TextRun({
        text: text,
        font: "Times New Roman",
        size: 22, // 11pt
        bold: true,
      }),
    ],
  });
}

function bodyPara(text, opts = {}) {
  return new Paragraph({
    alignment: AlignmentType.JUSTIFIED,
    spacing: { after: 60, line: 260 },
    indent: opts.indent ? { firstLine: 280 } : undefined,
    children: [
      new TextRun({
        text: text,
        font: "Times New Roman",
        size: 19, // 9.5pt
        ...(opts.italic ? { italics: true } : {}),
      }),
    ],
  });
}

function bodyParaMulti(runs) {
  return new Paragraph({
    alignment: AlignmentType.JUSTIFIED,
    spacing: { after: 60, line: 260 },
    indent: { firstLine: 280 },
    children: runs.map(r => new TextRun({
      text: r.text,
      font: "Times New Roman",
      size: 19, // 9.5pt
      bold: r.bold || false,
      italics: r.italic || false,
    })),
  });
}

function bibEntry(text) {
  return new Paragraph({
    alignment: AlignmentType.LEFT,
    spacing: { after: 40 },
    indent: { left: 360, hanging: 360 },
    children: [
      new TextRun({
        text: text,
        font: "Times New Roman",
        size: 16, // 8pt
      }),
    ],
  });
}

// ---- Build Document ----

// Section 1: Single column header (title, authors, abstract)
const headerSection = {
  properties: {
    page: {
      size: { width: 11906, height: 16838 }, // A4
      margin: { top: 1134, right: 1134, bottom: 1134, left: 1134 }, // ~2cm margins
    },
    pageNumbers: { start: 1, formatType: NumberFormat.DECIMAL },
  },
  headers: { default: new Header({ children: [] }) },
  footers: { default: makeFooter() },
  children: [
    // Title
    new Paragraph({
      alignment: AlignmentType.CENTER,
      spacing: { after: 120 },
      children: [
        new TextRun({
          text: title,
          font: "Times New Roman",
          size: 44, // 22pt
        }),
      ],
    }),
    // Authors
    new Paragraph({
      alignment: AlignmentType.CENTER,
      spacing: { after: 40 },
      children: [
        new TextRun({
          text: authors,
          font: "Times New Roman",
          size: 22, // 11pt
        }),
      ],
    }),
    // Affiliation
    new Paragraph({
      alignment: AlignmentType.CENTER,
      spacing: { after: 120 },
      children: [
        new TextRun({
          text: affiliation,
          font: "Times New Roman",
          size: 16, // 8pt
        }),
      ],
    }),
    // Abstract heading
    new Paragraph({
      alignment: AlignmentType.CENTER,
      spacing: { before: 80, after: 60 },
      children: [
        new TextRun({
          text: "Abstract",
          font: "Times New Roman",
          size: 19, // 9.5pt
          bold: true,
        }),
      ],
    }),
    // Abstract body
    new Paragraph({
      alignment: AlignmentType.JUSTIFIED,
      spacing: { after: 120, line: 240 },
      children: [
        new TextRun({
          text: abstractText,
          font: "Times New Roman",
          size: 17, // 8.5pt
          italics: true,
        }),
      ],
    }),
  ],
};

// Section 2: Two-column body
const bodySection = {
  properties: {
    type: SectionType.CONTINUOUS,
    column: {
      space: 360,
      count: 2,
      equalWidth: true,
    },
  },
  children: [
    // 1. SHORT SUMMARY
    heading1("1. Summary of Contribution"),

    bodyPara("RISCVML (riscvml.com) is an educational platform that provides a structured, hands-on curriculum for learning Rust-based embedded systems development on RISC-V microcontrollers. The platform targets the full range of Espressif's RISC-V SoCs: the entry-level ESP32-C3, the Wi-Fi 6-capable ESP32-C6, the TTGO T-Beam (a LoRa-capable development board), and the high-performance ESP32-P4 — a dual-core 400 MHz RISC-V processor with AI instruction extensions, 128-bit vector operations, MIPI-CSI/DSI camera and display interfaces, and H.264 hardware encoding."),

    bodyPara("The curriculum comprises 172 chapters organized into seven progressive modules. Modules 1\u20135 cover Rust fundamentals, GPIO/sensor/peripheral control (I2C, SPI, PCA9685), power management and solar harvesting, LoRa on T-Beam, and multi-device interconnection via ESP-NOW/MQTT. Module 7 bridges firmware to desktop applications using Tauri.", true),
    bodyParaMulti([
      { text: "Module 6 ", bold: true },
      { text: "is dedicated to the ESP32-P4, with Rust exercises for each advanced subsystem: type-safe ISP camera pipeline configuration (MIPI-CSI capture, white balance, demosaicing), hardware-accelerated 2D rendering via PPA with LVGL bindings, end-to-end H.264 encoding at 1080p@30fps with zero-copy buffer management, async DMA orchestration via embassy with compile-time borrow checking, and vectorized ML inference using 128-bit SIMD inline assembly wrappers around the RISC-V vector ISA." },
    ]),

    heading2("Capstone: Bird Detection Pipeline"),

    bodyPara("Module 6 culminates in a real-world capstone: a complete bird-detection system on ESP32-P4 that exercises every hardware subsystem in a single, deployable application. The pipeline is Rust-first while leveraging ESP-IDF drivers (esp-video, esp-detection/esp-dl) via FFI, demonstrating pragmatic interoperability:"),

    bodyParaMulti([
      { text: "Phase 1 \u2014 Camera \u2192 ISP \u2192 Display: ", bold: true },
      { text: "MIPI-CSI capture at 30\u201360 FPS using the Camera Controller Driver, ISP pipeline (white balance, auto-exposure, demosaicing) via type-safe Rust abstractions, DMA-driven frame transfer to MIPI-DSI display, PPA for scaling/rotation. Embassy async runtime orchestrates zero-copy buffer flow enforced by ownership semantics." },
    ]),

    bodyParaMulti([
      { text: "Phase 2 \u2014 On-Device Inference: ", bold: true },
      { text: "Espressif\u2019s esp-detection/esp-dl runs quantized object detection on the P4\u2019s AI-accelerated cores. PPA hardware-downscales frames to model input resolution, avoiding CPU-bound resizing. Bounding boxes and confidence scores overlay the preview via PPA alpha blending. The 128-bit vector extensions accelerate tensor operations within esp-dl. Detection results (species, confidence, bounding box, timestamp) are logged to a SQLite3 database (riscvml_detect.db). An RGB LED is driven to a species-specific color via a bird_led_colors lookup table in the same database, providing instant visual identification of detected birds." },
    ]),

    bodyParaMulti([
      { text: "Phase 3 \u2014 Tracking, Servos & Recording: ", bold: true },
      { text: "Detection coordinates drive PCA9685 pan/tilt servos (reusing Module 2 abstractions). H.264 hardware encoder records events at 1080p@30fps with zero-copy frame ingestion. A companion ESP32-C6 provides Wi-Fi 6 via ESP-Hosted/SDIO for MQTT alerts and RTSP streaming \u2014 demonstrating the P4\u2019s intended companion-chip architecture." },
    ]),

    bodyPara("The bird detection capstone instantiates a reusable Detect \u2192 Visualize \u2192 React pattern: Sensor Input \u2192 ML Classification \u2192 SQLite Lookup (class \u2192 RGB color) \u2192 RGB LED \u2192 Reaction. The curriculum applies this same architecture to object/obstacle detection, plant health assessment, and sound classification \u2014 swapping sensors, models, and color maps while sharing the ESP32-P4 infrastructure, teaching transferable ML-on-edge skills. Each chapter functions as a self-contained 30\u201360 minute learning unit with complete source code, wiring diagrams, and expected output.", true),

    // 2. IMPORTANCE FOR COMMUNITY
    heading1("2. Importance for the Community"),

    bodyPara("The RISC-V ecosystem faces an asymmetric growth challenge: hardware availability has scaled rapidly — over 20 billion cores projected by 2025 — but developer education has not kept pace. Industry surveys consistently identify the software ecosystem as the primary barrier to RISC-V adoption. RISCVML addresses this with four differentiators:"),

    bodyParaMulti([
      { text: "Rust-first approach: ", bold: true },
      { text: "While most RISC-V tutorials rely on C/C++, RISCVML uses Rust throughout. Compile-time memory safety eliminates entire bug classes common in embedded C, and zero-cost abstractions are particularly valuable on resource-constrained RISC-V cores. The Rust embedded ecosystem (esp-hal, embassy) has matured sufficiently to make this production-viable." },
    ]),

    bodyParaMulti([
      { text: "Commercially available hardware: ", bold: true },
      { text: "The ESP32-C3 (~\u20AC3), ESP32-C6 (~\u20AC4), and ESP32-P4 dev boards (~\u20AC25) ensure accessibility across the performance spectrum \u2014 learners progress from low-cost modules to the P4\u2019s 400 MHz dual-core with AI extensions within a unified Espressif RISC-V ecosystem." },
    ]),

    bodyParaMulti([
      { text: "End-to-end pipeline: ", bold: true },
      { text: "From bare-metal firmware to Tauri desktop applications, and from minimal IoT nodes (C3) to multimedia edge computing (P4 with MIPI displays and cameras), RISCVML spans the full embedded spectrum." },
    ]),

    bodyParaMulti([
      { text: "Character-driven engagement: ", bold: true },
      { text: "The platform\u2019s mascot, Count Rusty von Risc-V (\u201CRusty-V\u201D), embodies the technology stack \u2014 a Rust crab body, RISC-V chip chest, ML neural-network brain dome \u2014 making complex terminology visually approachable for younger learners and career-changers entering the ecosystem." },
    ]),

    // 3. FOSTERING THE ECOSYSTEM
    heading1("3. Ecosystem Contribution"),

    bodyPara("RISCVML contributes at multiple levels: as a talent pipeline providing English-language, self-paced training comparable to China's SOPIC program; as implicit ecosystem testing through real-world exercises against Espressif's esp-rs toolchains with upstream bug reports; and as a freemium distribution model (\u20AC2/chapter, \u20AC30 bundle, free introductory content) that sustains development while remaining accessible. The platform also offers partnership opportunities for ecosystem companies seeking developer onboarding content."),

    // 4. TARGET AUDIENCE
    heading1("4. Target Audience"),

    bodyPara("The primary audiences are: embedded engineers evaluating ARM-to-RISC-V transition with Rust; university educators seeking structured RISC-V coursework on affordable hardware; IoT/LoRa hobbyists wanting guided RISC-V learning paths; younger learners and career-changers drawn in by the platform\u2019s character-driven, visually engaging approach; and ecosystem companies interested in educational partnerships or curriculum licensing."),

    bodyPara("The poster presentation will include QR codes linking to the live platform, sample chapter previews, and demonstrations of the firmware-to-desktop pipeline running on ESP32-C6 and ESP32-P4 hardware with MIPI display output.", true),

    // REFERENCES
    heading1("References"),

    bibEntry("[1] RISC-V International, \"RISC-V Ecosystem Overview,\" riscv.org, 2025."),
    bibEntry("[2] Espressif Systems, \"ESP32-C3, ESP32-C6, and ESP32-P4 Technical Reference Manuals,\" espressif.com, 2024-2025."),
    bibEntry("[3] The Embedded Rust Book, docs.rust-embedded.org, 2025."),
    bibEntry("[4] Espressif, \"esp-dl: Deep Learning Library for ESP32-S3/P4,\" github.com/espressif/esp-dl, 2025."),
    bibEntry("[5] Espressif, \"esp-video: Video Framework for ESP32-P4,\" github.com/espressif/esp-video-components, 2025."),
    bibEntry("[6] EE Times, \"RISC-V Exceeding Expectations in AI, China Deployment,\" October 2025."),
  ],
};

const doc = new Document({
  sections: [headerSection, bodySection],
});

Packer.toBuffer(doc).then((buffer) => {
  fs.writeFileSync(__dirname + "/RISCVML-Summit-Europe-2026-Submission.docx", buffer);
  console.log("Document created successfully!");
});
