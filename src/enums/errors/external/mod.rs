use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

use super::internal;
use crate::dto::response::error::Response;

mod auth;
mod device;
mod session;

pub use auth::Auth;
pub use device::Device;
pub use session::Session;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum Error {
    Auth(Auth),
    Session(Session),
    Device(Device),
    Serialization,
    Internal,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Auth(e) => write!(f, "{}", e),
            Error::Session(e) => write!(f, "{}", e),
            Error::Device(e) => write!(f, "{}", e),
            Error::Serialization => write!(f, "SerializationError"),
            Error::Internal => write!(f, "InternalError"),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> response::Response {
        match self {
            Error::Auth(e) => e.into_response(),
            Error::Session(e) => e.into_response(),
            Error::Device(e) => e.into_response(),
            _ => Response {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal server error".to_string(),
                error: "Internal",
            }
            .into_response(),
        }
    }
}

impl From<internal::Error> for Error {
    fn from(error: internal::Error) -> Self {
        match error {
            internal::Error::Auth(e) => Error::Auth(e.into()),
            internal::Error::Session(e) => Error::Session(e.into()),
            internal::Error::Device(e) => Error::Device(e.into()),
            _ => Error::Internal,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
