use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum DeviceError {
    DeviceNotFound,
}

impl IntoResponse for DeviceError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            DeviceError::DeviceNotFound => (StatusCode::NOT_FOUND, "Device not found"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
