use crate::dto::response::error::Response;
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
