use axum::{
    extract::State,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{mysql::MySqlPoolOptions, FromRow, MySql, Pool};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load .env file
    dotenvy::dotenv().ok();
    
    // 2. Construct Database URL from existing env vars
    let db_user = env::var("DB_USER").unwrap_or_else(|_| "root".to_string());
    let db_pass = env::var("DB_PASSWORD").unwrap_or_else(|_| "".to_string());
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "host.docker.internal".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "3306".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "monitoring_db".to_string());

    let database_url = format!("mysql://{}:{}@{}:{}/{}", db_user, db_pass, db_host, db_port, db_name);

    // 3. Connect to Database
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("❌ Failed to connect to database. Check your Docker network or .env vars.");

    println!("✅ Connected to MySQL at {}", db_host);

    // 4. Setup Router with CORS
    let app = Router::new()
        .route("/api/status", get(get_status))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // 5. Start Server
    let port_str = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port_str).parse()?;
    
    println!("🚀 Monitor API running on port {}", port_str);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
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

// --- Logic ---

async fn get_status(State(pool): State<Pool<MySql>>) -> Json<Vec<ServiceResponse>> {
    // Fetch all rows
    let rows: Vec<ServiceRow> = sqlx::query_as("SELECT * FROM service_status")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Database Query Error: {}", e);
            vec![] 
        });

    let now = Utc::now();

    let processed: Vec<ServiceResponse> = rows.into_iter().map(|row| {
        // Time diff logic
        let diff_seconds = (now - row.last_update).num_seconds();

        // Status Logic
        let display_status = if row.status == "down" {
            "outage".to_string()
        } else if diff_seconds > 120 {
            "stale".to_string()
        } else {
            "operational".to_string()
        };

        // Hex ID Logic (Replaces Buffer.from...)
        let raw_id = format!("svc-{}", row.id);
        let hex_full = hex::encode(raw_id);
        let display_id = if hex_full.len() >= 8 {
            hex_full[0..8].to_string()
        } else {
            hex_full
        };

        ServiceResponse {
            id: row.id,
            display_id,
            name: row.project_name,
            last_update: row.last_update,
            status: display_status,
        }
    }).collect();

    Json(processed)
}