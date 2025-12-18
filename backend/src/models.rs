use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Monitor {
    pub id: i32,
    pub service: String,
    pub instance: String,
    pub description: String,
    pub last_update: DateTime<chrono::Utc>,
    pub vps: String,
}

// Matches Frontend JSON Expectation
#[derive(Serialize)]
pub struct ServiceResponse {
    pub id: i32,
    #[serde(rename = "displayId")] // JS expects camelCase
    pub display_id: String,
    pub name: String,
    pub instance: String,
    pub last_update: DateTime<chrono::Utc>,
    pub status: String,
    pub vps: String,
}