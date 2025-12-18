use sqlx::MySqlPool;

use crate::{errors::AppError, models::Monitor};

#[derive(Clone)]
pub struct MonitorRepository {
    pool: MySqlPool,
}

impl MonitorRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Monitor>, AppError> {
        let records = sqlx::query_as::<_, Monitor>(
            "SELECT id, project_name, last_update, status FROM monitors",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records)
    }
}
