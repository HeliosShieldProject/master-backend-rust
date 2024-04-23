use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    WrongToken,
    WrongPassword,
    WrongEmail,
    MissingCredentials,
    MissingDevice,
    TokenCreation,
    UserNotFound,
    UserAlreadyExists,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongToken => (StatusCode::BAD_REQUEST, "Wrong token"),
            AuthError::WrongPassword => (StatusCode::BAD_REQUEST, "Wrong password"),
            AuthError::WrongEmail => (StatusCode::BAD_REQUEST, "Wrong email"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::MissingDevice => (StatusCode::BAD_REQUEST, "Missing device"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AuthError::UserAlreadyExists => (StatusCode::BAD_REQUEST, "User already exists"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
