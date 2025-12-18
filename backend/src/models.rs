use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Monitor {
    pub id: i32,
    pub project_name: String,
    pub last_update: DateTime<chrono::Utc>,
    pub status: String,
}