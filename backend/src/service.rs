use axum::Json;

use crate::{errors::AppError, models::ServiceResponse, repositories::MonitorRepository};

#[derive(Clone)]
pub struct MonitorService {
    repo: MonitorRepository,
}

impl MonitorService {
    pub fn new(repo: MonitorRepository) -> Self {
        Self { repo }
    }

    pub async fn get_all_monitors(&self) -> Result<Json<Vec<ServiceResponse>>, AppError> {
        let rows = self.repo.find_all().await?;

        let now = chrono::Utc::now();
        let processed: Vec<ServiceResponse> = rows
            .into_iter()
            .map(|row| {
                // Time diff logic
                let diff_seconds = (now - row.last_update).num_seconds();

                // Status Logic
                let display_status = if diff_seconds > 120 {
                    // If no heartbeat for > 2 mins, it's DOWN
                    "outage".to_string()
                } else if diff_seconds > 60 {
                    // If no heartbeat for > 1 min (but < 2 mins), it's STALE
                    "stale".to_string()
                } else {
                    // If heartbeat is within the last 60 seconds, it's OPERATIONAL
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
                    name: row.service,
                    instance: row.instance,
                    vps: row.vps,
                    last_update: row.last_update,
                    status: display_status,
                }
            })
            .collect();

        Ok(Json(processed))
    }
}
