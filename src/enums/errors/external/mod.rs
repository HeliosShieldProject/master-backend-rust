mod auth_error;
pub use auth_error::AuthError;

mod device_error;
pub use device_error::DeviceError;

mod country_error;
pub use country_error::CountryError;

mod session_error;
pub use session_error::SessionError;

use crate::dto::response::error::Response;
use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

use super::internal::InternalError;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum ExternalError {
    AuthError(AuthError),
    DeviceError(DeviceError),
    CountryError(CountryError),
    SessionError(SessionError),
    SerializationError,
    Internal,
}

impl std::fmt::Display for ExternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalError::AuthError(e) => write!(f, "{}", e),
            ExternalError::DeviceError(e) => write!(f, "{}", e),
            ExternalError::CountryError(e) => write!(f, "{}", e),
            ExternalError::SessionError(e) => write!(f, "{}", e),
            ExternalError::SerializationError => write!(f, "Serialization error"),
            ExternalError::Internal => write!(f, "Internal server error"),
        }
    }
}

impl IntoResponse for ExternalError {
    fn into_response(self) -> response::Response {
        match self {
            ExternalError::AuthError(e) => e.into_response(),
            ExternalError::DeviceError(e) => e.into_response(),
            ExternalError::CountryError(e) => e.into_response(),
            ExternalError::SessionError(e) => e.into_response(),
            _ => Response {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal server error".to_string(),
                error: "Internal",
            }
            .into_response(),
        }
    }
}

impl From<InternalError> for ExternalError {
    fn from(error: InternalError) -> Self {
        match error {
            InternalError::AuthError(e) => ExternalError::AuthError(e.into()),
            InternalError::DeviceError(e) => ExternalError::DeviceError(e.into()),
            InternalError::CountryError(e) => ExternalError::CountryError(e.into()),
            InternalError::SessionError(e) => ExternalError::SessionError(e.into()),
            _ => ExternalError::Internal,
        }
    }
}
