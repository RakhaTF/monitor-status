use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    // We can add more later, e.g., NotFound, AuthError, etc.
}

// 1. Tell Axum how to convert our Error into a Response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Map the error type to a status code and message
        let (status, error_message) = match self {
            AppError::Database(err) => {
                // In production, don't expose 'err' directly to users!
                // Log it here instead using tracing
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".to_string(),
                )
            }
        };

        // Create the JSON body
        let body = Json(json!({
            "error": error_message,
        }));

        // Return the tuple (Status, Body)
        (status, body).into_response()
    }
}

// 2. Enable the '?' operator
// This allows us to use '?' on sqlx errors directly
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}
