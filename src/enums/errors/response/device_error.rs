use crate::dto::response::error::ErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub enum DeviceError {
    DeviceNotFound,
    DeviceAlreadyExists,
}

impl IntoResponse for DeviceError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            DeviceError::DeviceNotFound => (StatusCode::NOT_FOUND, "Device not found"),
            DeviceError::DeviceAlreadyExists => (StatusCode::BAD_REQUEST, "Device already exists"),
        };

        ErrorResponse {
            status,
            message: message.to_string(),
            error: self,
        }
        .into_response()
    }
}

impl DeviceError {
    pub fn to_string(&self) -> String {
        match self {
            DeviceError::DeviceNotFound => "Device not found".to_string(),
            DeviceError::DeviceAlreadyExists => "Device already exists".to_string(),
        }
    }
}