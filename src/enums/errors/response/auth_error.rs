use crate::dto::response::error::Response;
use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
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

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::WrongToken => write!(f, "WrongToken"),
            AuthError::WrongPassword => write!(f, "WrongPassword"),
            AuthError::WrongEmail => write!(f, "WrongEmail"),
            AuthError::MissingCredentials => write!(f, "MissingCredentials"),
            AuthError::MissingDevice => write!(f, "MissingDevice"),
            AuthError::TokenCreation => write!(f, "TokenCreation"),
            AuthError::UserNotFound => write!(f, "UserNotFound"),
            AuthError::UserAlreadyExists => write!(f, "UserAlreadyExists"),
            AuthError::PasswordIsSame => write!(f, "PasswordIsSame"),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> response::Response {
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

        Response {
            status,
            message: message.to_string(),
            error: self,
        }
        .into_response()
    }
}
