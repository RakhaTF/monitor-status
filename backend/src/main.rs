use axum::{routing::get, Router};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{mysql::MySqlPoolOptions, FromRow};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

// Logging imports
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod infra;
mod state;
mod errors;
mod models;
mod handlers;
mod repositories;
mod service;

use crate::{config::AppConfig, infra::MonitoringDb, state::AppState};
use infra::factory::DbRegistry;
use handlers::{get_projects};

#[tokio::main]
async fn main() {
    // Initialize Tracing Subscriber for logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rust_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 1. Load .env file
    dotenvy::dotenv().ok();

    let config = AppConfig::from_env();

    let registry = DbRegistry::boot(&config).await;

    let state = AppState {
        monitoring_db: MonitoringDb(registry.monitoring_db),
    };

    // 2. Construct Database URL from existing env vars
    let db_user = env::var("DB_USER").unwrap_or_else(|_| "root".to_string());
    let db_pass = env::var("DB_PASSWORD").unwrap_or_else(|_| "".to_string());
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "host.docker.internal".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "3306".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "monitoring_db".to_string());

    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_pass, db_host, db_port, db_name
    );

    // 3. Connect to Database
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database. Check your Docker network or .env vars.");

    println!("Connected to MySQL at {}", db_host);

    // 4. Setup Router with CORS
    let app = Router::new()
        .route("/api/status", get(get_projects))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // 5. Start Server
    let port_str = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port_str).parse().unwrap();

    println!("Monitor API running on port {}", port_str);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// --- Data Structures ---

// Matches SQL table: `service_status`
#[derive(FromRow)]
struct ServiceRow {
    id: i32,
    project_name: String,
    last_update: DateTime<Utc>,
    status: String,
}

// Matches Frontend JSON Expectation
#[derive(Serialize)]
struct ServiceResponse {
    id: i32,
    #[serde(rename = "displayId")] // JS expects camelCase
    display_id: String,
    name: String,
    last_update: DateTime<Utc>,
    status: String,
}