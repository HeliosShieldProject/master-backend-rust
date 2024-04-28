use crate::dto::response::error::ErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub enum SessionError {
    SessionNotFound,
    SessionAlreadyExists,
}

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            SessionError::SessionNotFound => (StatusCode::NOT_FOUND, "Session not found"),
            SessionError::SessionAlreadyExists => {
                (StatusCode::BAD_REQUEST, "Session already exists")
            }
        };

        ErrorResponse {
            status,
            message: message.to_string(),
            error: self,
        }
        .into_response()
    }
}
