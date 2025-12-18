use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::time::Duration;
use tracing::info;

// 1. Define a Config Struct (similar to your `options` in TS)
pub struct DbConfig<'a> {
    pub host: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub database: &'a str,
    pub port: u16,
    pub pool_size: u16,
}

use crate::{config::AppConfig, errors::AppError};

// 2. The equivalent of `CreateDataSource`
// We make this private (not pub) because only this module needs to know how to build it.
async fn create_data_source(cfg: DbConfig<'_>) -> Result<MySqlPool, AppError> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        cfg.username, cfg.password, cfg.host, cfg.port, cfg.database
    );

    let pool = MySqlPoolOptions::new()
        .max_connections(cfg.pool_size.into())
        .acquire_timeout(Duration::from_secs(5))
        .connect(&url)
        .await?;

    info!("Created DataSource for: {}", cfg.database);
    Ok(pool)
}

// 3. The "Boot" logic (Health Checks)
async fn health_check(pool: &MySqlPool, name: &str) {
    match sqlx::query("SELECT 1=1").execute(pool).await {
        Ok(_) => info!("Health Check Passed: {}", name),
        Err(e) => panic!("Health Check Failed for {}: {}", name, e),
    }
}

// 4. Container for your specific databases
// This replaces your logic of "looping through datasources to find TF or Porto"
// In Rust, we just store them in named fields so we don't have to search.
#[derive(Clone)]
pub struct DbRegistry {
    pub monitoring_db: MySqlPool,
}

impl DbRegistry {
    // This is your `BootMySQL` function
    pub async fn boot(config: &AppConfig) -> Self {
        info!("event: application/boot/mysql, msg: Booting...");

        // A. Create Monitoring DataSource
        // (In a real app, load these strings from env::var)
        let monitoring_pool = create_data_source(DbConfig {
            host: &config.monitoring_host,
            username: &config.monitoring_user,
            password: &config.monitoring_pass,
            database: &config.monitoring_name,
            port: config.monitoring_port,
            pool_size: config.monitoring_pool_size,
        })
        .await
        .expect("Failed to create Monitoring DataSource");

        health_check(&monitoring_pool, "Monitoring Database").await;

        info!("event: application/boot/mysql, msg: Done...");

        Self {
            monitoring_db: monitoring_pool,
        }
    }
}
