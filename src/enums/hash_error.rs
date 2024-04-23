use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum HashError {
    HashError,
}

impl IntoResponse for HashError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            HashError::HashError => (StatusCode::INTERNAL_SERVER_ERROR, "Hash error"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
