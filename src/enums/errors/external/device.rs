use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

use crate::{dto::response::error::Response, enums::errors::internal};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum Device {
    NotFound,
    SelfRevocation,
    AlreadyRevoked,
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Device::NotFound => write!(f, "DeviceNotFound"),
            Device::SelfRevocation => write!(f, "DeviceSelfRevocation"),
            Device::AlreadyRevoked => write!(f, "DeviceAlreadyRevoked"),
        }
    }
}

impl IntoResponse for Device {
    fn into_response(self) -> response::Response {
        let (status, message) = match self {
            Device::NotFound => (StatusCode::NOT_FOUND, "Device not found"),
            Device::SelfRevocation => (
                StatusCode::FORBIDDEN,
                "Device self revocation. Use /auth/logout instead.",
            ),
            Device::AlreadyRevoked => (StatusCode::CONFLICT, "Device already revoked"),
        };

        Response {
            status,
            message: message.to_string(),
            error: self.to_string(),
        }
        .into_response()
    }
}

impl From<internal::Device> for Device {
    fn from(error: internal::Device) -> Self {
        match error {
            internal::Device::NotFound => Device::NotFound,
            internal::Device::SelfRevocation => Device::SelfRevocation,
            internal::Device::AlreadyRevoked => Device::AlreadyRevoked,
        }
    }
}
