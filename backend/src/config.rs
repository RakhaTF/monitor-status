use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    // Monitoring Database Configs
    pub monitoring_host: String,
    pub monitoring_user: String,
    pub monitoring_pass: String,
    pub monitoring_name: String,
    pub monitoring_port: u16,
    pub monitoring_pool_size: u16,
}

impl AppConfig {
    // Helper to load everything from environment
    pub fn from_env() -> Self {
        Self {
            monitoring_host: env::var("MONITORING_MYSQL_HOST").expect("MONITORING_MYSQL_HOST missing"),
            monitoring_user: env::var("MONITORING_MYSQL_USER").expect("MONITORING_MYSQL_USER missing"),
            monitoring_pass: env::var("MONITORING_MYSQL_PASSWORD").expect("MONITORING_MYSQL_PASSWORD missing"),
            monitoring_name: env::var("MONITORING_MYSQL_DATABASE").expect("MONITORING_MYSQL_DATABASE missing"),
            monitoring_port: env::var("MONITORING_MYSQL_PORT")
                .unwrap_or_else(|_| "3306".into())
                .parse()
                .expect("Failed to parse MONITORING_MYSQL_PORT"),
            monitoring_pool_size: env::var("MONITORING_MYSQL_POOL_SIZE")
                .unwrap_or_else(|_| "10".into())
                .parse()
                .expect("Failed to parse MONITORING_MYSQL_POOL_SIZE"),
        }
    }
}