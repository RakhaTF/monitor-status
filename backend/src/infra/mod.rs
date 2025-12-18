// This makes factory.rs accessible to the rest of the app
pub mod factory; 

use sqlx::MySqlPool;

// wrappers
#[derive(Clone)]
pub struct MonitoringDb(pub MySqlPool);