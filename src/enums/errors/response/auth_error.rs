use crate::dto::response::error::ErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub enum AuthError {
    WrongToken,
    WrongPassword,
    WrongEmail,
    MissingCredentials,
    MissingDevice,
    TokenCreation,
    UserNotFound,
    UserAlreadyExists,
    PasswordIsSame,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::WrongToken => (StatusCode::UNAUTHORIZED, "Wrong token"),
            AuthError::WrongPassword => (StatusCode::UNAUTHORIZED, "Wrong password"),
            AuthError::WrongEmail => (StatusCode::UNAUTHORIZED, "Wrong email"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::MissingDevice => (StatusCode::BAD_REQUEST, "Missing device"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AuthError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthError::PasswordIsSame => (StatusCode::CONFLICT, "Password is the same"),
        };

        ErrorResponse {
            status,
            message: message.to_string(),
            error: self,
        }
        .into_response()
    }
}

impl AuthError {
    pub fn to_string(&self) -> String {
        match self {
            AuthError::WrongToken => "Wrong token".to_string(),
            AuthError::WrongPassword => "Wrong password".to_string(),
            AuthError::WrongEmail => "Wrong email".to_string(),
            AuthError::MissingCredentials => "Missing credentials".to_string(),
            AuthError::MissingDevice => "Missing device".to_string(),
            AuthError::TokenCreation => "Token creation error".to_string(),
            AuthError::UserNotFound => "User not found".to_string(),
            AuthError::UserAlreadyExists => "User already exists".to_string(),
            AuthError::PasswordIsSame => "Password is same".to_string(),
        }
    }
}