use axum::extract::FromRef;
use crate::infra::MonitoringDb;

#[derive(Clone)]
pub struct AppState {
    pub monitoring_db: MonitoringDb,
}

impl FromRef<AppState> for MonitoringDb {
    fn from_ref(state: &AppState) -> Self {
        state.monitoring_db.clone()
    }
}