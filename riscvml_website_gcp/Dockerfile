FROM rust:1.85 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ src/
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/riscvml_website_gcp .
COPY static/ static/
ENV PORT=8080
EXPOSE 8080
CMD ["./riscvml_website_gcp"]
