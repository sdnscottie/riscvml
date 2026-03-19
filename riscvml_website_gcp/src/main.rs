use axum::{
    Router,
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::Serialize;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    services::ServeDir,
};
use tracing_subscriber::EnvFilter;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct ModuleInfo {
    id: u8,
    title: &'static str,
    chapters: u16,
    hardware: &'static str,
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

async fn api_modules() -> Json<Vec<ModuleInfo>> {
    Json(vec![
        ModuleInfo { id: 1, title: "Rust Fundamentals & GPIO", chapters: 25, hardware: "ESP32-C3", status: "available" },
        ModuleInfo { id: 2, title: "I2C, Sensors & Motor Control", chapters: 85, hardware: "ESP32-C3/C6", status: "available" },
        ModuleInfo { id: 3, title: "Power Management & Solar", chapters: 20, hardware: "ESP32-C3", status: "available" },
        ModuleInfo { id: 4, title: "LoRa & Long-Range Comms", chapters: 15, hardware: "TTGO T-Beam", status: "in-progress" },
        ModuleInfo { id: 5, title: "Multi-Device: ESP-NOW & MQTT", chapters: 12, hardware: "ESP32-C6", status: "in-progress" },
        ModuleInfo { id: 6, title: "ESP32-P4 High-Performance", chapters: 10, hardware: "ESP32-P4", status: "in-progress" },
        ModuleInfo { id: 7, title: "Firmware-to-Desktop via Tauri", chapters: 5, hardware: "All", status: "planned" },
    ])
}

async fn fallback() -> impl IntoResponse {
    // SPA fallback: serve index.html for any unknown route
    // The SPA's client-side router handles the path
    match tokio::fs::read_to_string("static/index.html").await {
        Ok(html) => (StatusCode::OK, [("content-type", "text/html")], html).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let app = Router::new()
        // API routes
        .route("/api/health", get(health))
        .route("/api/modules", get(api_modules))
        // Static files (CSS, JS, images)
        .nest_service("/static", ServeDir::new("static"))
        // SPA fallback — serves index.html for all other routes
        .fallback(get(fallback))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive());

    let addr = format!("0.0.0.0:{port}");
    tracing::info!("riscvml.org listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
