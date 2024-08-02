use super::ExternalError;
use crate::{dto::response::error::Response, enums::errors::internal};
use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum SessionError {
    SessionNotFound,
    SessionAlreadyExists,
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::SessionNotFound => write!(f, "SessionNotFound"),
            SessionError::SessionAlreadyExists => write!(f, "SessionAlreadyExists"),
        }
    }
}

impl IntoResponse for SessionError {
    fn into_response(self) -> response::Response {
        let (status, message) = match self {
            SessionError::SessionNotFound => (StatusCode::NOT_FOUND, "Session not found"),
            SessionError::SessionAlreadyExists => {
                (StatusCode::BAD_REQUEST, "Session already exists")
            }
        };

        Response {
            status,
            message: message.to_string(),
            error: self,
        }
        .into_response()
    }
}

impl From<internal::SessionError> for SessionError {
    fn from(error: internal::SessionError) -> Self {
        match error {
            internal::SessionError::SessionNotFound => SessionError::SessionNotFound,
            internal::SessionError::SessionAlreadyExists => SessionError::SessionAlreadyExists,
        }
    }
}
