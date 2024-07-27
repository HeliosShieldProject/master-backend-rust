use crate::dto::response::error::Response;
use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum DeviceError {
    DeviceNotFound,
    DeviceAlreadyExists,
}

impl IntoResponse for DeviceError {
    fn into_response(self) -> response::Response {
        let (status, message) = match self {
            DeviceError::DeviceNotFound => (StatusCode::NOT_FOUND, "Device not found"),
            DeviceError::DeviceAlreadyExists => (StatusCode::BAD_REQUEST, "Device already exists"),
        };

        Response {
            status,
            message: message.to_string(),
            error: self,
        }
        .into_response()
    }
}
