use crate::{dto::response::error::Response, enums::errors::internal};
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

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceError::DeviceNotFound => write!(f, "DeviceNotFound"),
            DeviceError::DeviceAlreadyExists => write!(f, "DeviceAlreadyExists"),
        }
    }
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

impl From<internal::DeviceError> for DeviceError {
    fn from(error: internal::DeviceError) -> Self {
        match error {
            internal::DeviceError::DeviceNotFound => DeviceError::DeviceNotFound,
            internal::DeviceError::DeviceAlreadyExists => DeviceError::DeviceAlreadyExists,
        }
    }
}
