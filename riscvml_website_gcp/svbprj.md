# riscvml_website_gcp

**Author:** Scottie von Bruchhausen (scottie@riscvml.org)
**Stack:** Rust (Axum 0.8) + Tokio + static SPA
**Deploy:** GCP Cloud Run (Docker container)
**Domain:** riscvml.org / riscvml.com

## Overview

Rust SPA webserver for riscvml.org. Axum serves both the API and static frontend.
Single binary, single container, deploys to GCP Cloud Run.

## Architecture

```
Browser → GCP Cloud Run → Axum (Rust)
                            ├── /api/health     → JSON health check
                            ├── /api/modules    → curriculum module list
                            ├── /static/*       → CSS, JS, images
                            └── /*              → SPA fallback (index.html)
```

## Build & Run

```bash
# Local development
cargo run
# → http://localhost:8080

# With custom port
PORT=3000 cargo run

# Docker build
docker build -t riscvml-web .
docker run -p 8080:8080 riscvml-web

# Deploy to GCP Cloud Run
gcloud run deploy riscvml-web \
  --source . \
  --region europe-west1 \
  --allow-unauthenticated \
  --port 8080
```

## Project Structure

```
riscvml_website_gcp/
├── svbprj.md                          ← this file
├── Cargo.toml                         ← Axum + Tokio + tower-http
├── Dockerfile                         ← multi-stage build for Cloud Run
├── src/
│   └── main.rs                        ← Axum server: API routes + SPA fallback
├── static/
│   └── index.html                     ← SPA frontend (vanilla JS, no framework)
├── docs_about__riscvml_website_gcp/   ← architecture diagrams
├── anki_refs/                         ← flashcard cross-references
└── pics/                              ← screenshots, deploy evidence
```

## API Routes

| Method | Path | Response |
|--------|------|----------|
| GET | `/api/health` | `{"status":"ok","version":"0.1.0"}` |
| GET | `/api/modules` | Array of curriculum modules with chapters, hardware, status |
| GET | `/static/*` | Static files (CSS, JS, images) |
| GET | `/*` | SPA fallback → index.html |

## GCP Cloud Run Config

- **Region:** europe-west1 (Belgium — close to Germany)
- **CPU:** 1 vCPU (Axum is lightweight)
- **Memory:** 256MB
- **Min instances:** 0 (scale to zero when idle)
- **Max instances:** 10
- **Concurrency:** 80 (Tokio handles this easily)
- **Port:** 8080 (Cloud Run default)

## Frontend

Vanilla HTML/CSS/JS SPA — no build step, no npm, no framework.
Color scheme: Rust orange (#CE422B), RISC-V green (#2D9B3A), ML purple (#7B4BB3).
Dark theme (#0D0D1A background).

Fetches module data from `/api/modules` on load. Client-side routing via SPA fallback.
