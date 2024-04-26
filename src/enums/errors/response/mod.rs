mod auth_error;
pub use auth_error::AuthError;

mod device_error;
pub use device_error::DeviceError;

use super::internal;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum ResponseError {
    AuthError(AuthError),
    DeviceError(DeviceError),
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
            internal::InternalError::AuthError(e) => match e {
                internal::AuthError::WrongEmail => ResponseError::AuthError(AuthError::WrongEmail),
                internal::AuthError::WrongPassword => {
                    ResponseError::AuthError(AuthError::WrongPassword)
                }
                internal::AuthError::PasswordIsSame => {
                    ResponseError::AuthError(AuthError::PasswordIsSame)
                }
                internal::AuthError::UserAlreadyExists => {
                    ResponseError::AuthError(AuthError::UserAlreadyExists)
                }
                internal::AuthError::TokenCreation => {
                    ResponseError::AuthError(AuthError::TokenCreation)
                }
                internal::AuthError::WrongToken => ResponseError::AuthError(AuthError::WrongToken),
                internal::AuthError::MissingDevice => {
                    ResponseError::AuthError(AuthError::MissingDevice)
                }
                internal::AuthError::MissingCredentials => {
                    ResponseError::AuthError(AuthError::MissingCredentials)
                }
                internal::AuthError::UserNotFound => {
                    ResponseError::AuthError(AuthError::UserNotFound)
                }
            },
            internal::InternalError::DeviceError(e) => match e {
                internal::DeviceError::DeviceAlreadyExists => {
                    ResponseError::DeviceError(DeviceError::DeviceAlreadyExists)
                }
                internal::DeviceError::DeviceNotFound => {
                    ResponseError::DeviceError(DeviceError::DeviceNotFound)
                }
            },
            internal::InternalError::HashError(e) => match e {
                internal::HashError::Hash => ResponseError::Internal,
                internal::HashError::Verify => ResponseError::AuthError(AuthError::WrongPassword),
            },
            internal::InternalError::TokenError(e) => match e {
                internal::TokenError::Encode => ResponseError::AuthError(AuthError::TokenCreation),
            },
            &internal::InternalError::UuidParse => ResponseError::Internal,
            &internal::InternalError::Internal => ResponseError::Internal,
        }
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ResponseError::AuthError(e) => match e {
                AuthError::WrongEmail => (StatusCode::BAD_REQUEST, "Wrong email"),
                AuthError::WrongPassword => (StatusCode::BAD_REQUEST, "Wrong password"),
                AuthError::PasswordIsSame => (StatusCode::BAD_REQUEST, "Password is the same"),
                AuthError::UserAlreadyExists => (StatusCode::BAD_REQUEST, "User already exists"),
                AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation"),
                AuthError::WrongToken => (StatusCode::BAD_REQUEST, "Wrong token"),
                AuthError::MissingDevice => (StatusCode::BAD_REQUEST, "Missing device"),
                AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
                AuthError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            },
            ResponseError::DeviceError(e) => match e {
                DeviceError::DeviceAlreadyExists => {
                    (StatusCode::BAD_REQUEST, "Device already exists")
                }
                DeviceError::DeviceNotFound => (StatusCode::NOT_FOUND, "Device not found"),
            },
            ResponseError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
