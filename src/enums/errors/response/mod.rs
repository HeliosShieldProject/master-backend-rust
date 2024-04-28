mod auth_error;
pub use auth_error::AuthError;

mod device_error;
pub use device_error::DeviceError;

mod country_error;
pub use country_error::CountryError;

mod session_error;
pub use session_error::SessionError;

use crate::{dto::response::error::ErrorResponse, enums::errors::internal};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum ResponseError {
    AuthError(AuthError),
    DeviceError(DeviceError),
    CountryError(CountryError),
    SessionError(SessionError),
    Internal,
}

pub trait Error {
    fn as_response(&self) -> ResponseError;
}

pub fn to_response<T: Error>(error: T) -> ResponseError {
    error.as_response()
}

impl Error for internal::InternalError {
    fn as_response(&self) -> ResponseError {
        match self {
            internal::InternalError::HashError(e) => e.as_response(),
            internal::InternalError::TokenError(e) => e.as_response(),
            internal::InternalError::AuthError(e) => e.as_response(),
            internal::InternalError::DeviceError(e) => e.as_response(),
            internal::InternalError::CountryError(e) => e.as_response(),
            internal::InternalError::SessionError(e) => e.as_response(),
            _ => ResponseError::Internal,
        }
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        match self {
            ResponseError::AuthError(e) => e.into_response(),
            ResponseError::DeviceError(e) => e.into_response(),
            ResponseError::CountryError(e) => e.into_response(),
            ResponseError::SessionError(e) => e.into_response(),
            _ => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal server error".to_string(),
                error: "Internal server error",
            }
            .into_response(),
        }
    }
}
