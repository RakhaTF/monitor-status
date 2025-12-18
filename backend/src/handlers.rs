use axum::{extract::State, http::StatusCode, response::IntoResponse};

use crate::errors::AppError;
use crate::infra::MonitoringDb;
use crate::repositories::MonitorRepository;
use crate::service::MonitorService;

pub async fn get_projects(
    State(MonitoringDb(pool)): State<MonitoringDb>,
) -> Result<impl IntoResponse, AppError> {
    // Dependency injection of the repository
    let repo = MonitorRepository::new(pool);
    let service = MonitorService::new(repo);

    let response = service.get_all_monitors().await?;

    Ok((StatusCode::OK, response))
}
