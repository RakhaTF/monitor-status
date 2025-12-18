use axum::{routing::get, Router};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

// Logging imports
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod errors;
mod handlers;
mod infra;
mod models;
mod repositories;
mod service;
mod state;

use crate::{config::AppConfig, infra::MonitoringDb, state::AppState};
use handlers::get_projects;
use infra::factory::DbRegistry;

#[tokio::main]
async fn main() {
    // Initialize Tracing Subscriber for logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "monitor_status_backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load .env file
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();

    // Boot Database
    let db = DbRegistry::boot(&config).await;

    let state = AppState {
        monitoring_db: MonitoringDb(db.monitoring_db),
    };

    // Setup Router with CORS
    let app = Router::new()
        .route("/api/status", get(get_projects))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // 5. Start Server
    let port_str = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port_str).parse().unwrap();

    tracing::info!("Monitor API running on port {}", port_str);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
