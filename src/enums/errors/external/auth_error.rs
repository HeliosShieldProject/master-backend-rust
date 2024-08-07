use crate::{dto::response::error::Response, enums::errors::internal};
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
    TokenCreation,
    UserNotFound,
    UserAlreadyExists,
    PasswordIsSame,
    OAuthFailed,
    OAuthDifferentEmail,
    NoClassicAuth,
    UnknownOAuthProvider,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::WrongToken => write!(f, "WrongToken"),
            AuthError::WrongPassword => write!(f, "WrongPassword"),
            AuthError::WrongEmail => write!(f, "WrongEmail"),
            AuthError::TokenCreation => write!(f, "TokenCreation"),
            AuthError::UserNotFound => write!(f, "UserNotFound"),
            AuthError::UserAlreadyExists => write!(f, "UserAlreadyExists"),
            AuthError::PasswordIsSame => write!(f, "PasswordIsSame"),
            AuthError::OAuthFailed => write!(f, "OAuthFailed"),
            AuthError::OAuthDifferentEmail => write!(f, "OAuthDifferentEmail"),
            AuthError::NoClassicAuth => write!(f, "NoClassicAuth"),
            AuthError::UnknownOAuthProvider => write!(f, "UnknownOAuthProvider"),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> response::Response {
        let (status, message) = match self {
            AuthError::WrongToken => (StatusCode::UNAUTHORIZED, "Wrong token"),
            AuthError::WrongPassword => (StatusCode::UNAUTHORIZED, "Wrong password"),
            AuthError::WrongEmail => (StatusCode::UNAUTHORIZED, "Wrong email"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AuthError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthError::PasswordIsSame => (StatusCode::CONFLICT, "Password is the same"),
            AuthError::OAuthFailed => (StatusCode::INTERNAL_SERVER_ERROR, "OAuth failed"),
            AuthError::OAuthDifferentEmail => (StatusCode::UNAUTHORIZED, "OAuth different email"),
            AuthError::NoClassicAuth => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "User has no classic auth",
            ),
            AuthError::UnknownOAuthProvider => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unknown OAuth provider")
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

impl From<internal::AuthError> for AuthError {
    fn from(error: internal::AuthError) -> Self {
        match error {
            internal::AuthError::WrongToken => AuthError::WrongToken,
            internal::AuthError::WrongPassword => AuthError::WrongPassword,
            internal::AuthError::WrongEmail => AuthError::WrongEmail,
            internal::AuthError::TokenCreation => AuthError::TokenCreation,
            internal::AuthError::UserNotFound => AuthError::UserNotFound,
            internal::AuthError::UserAlreadyExists => AuthError::UserAlreadyExists,
            internal::AuthError::PasswordIsSame => AuthError::PasswordIsSame,
            internal::AuthError::OAuthFailed => AuthError::OAuthFailed,
            internal::AuthError::OAuthDifferentEmail => AuthError::OAuthDifferentEmail,
            internal::AuthError::NoClassicAuth => AuthError::NoClassicAuth,
            internal::AuthError::UnknownOAuthProvider => AuthError::UnknownOAuthProvider,
        }
    }
}

impl From<internal::ReqwestError> for AuthError {
    fn from(error: internal::ReqwestError) -> Self {
        match error {
            internal::ReqwestError::JsonError => AuthError::OAuthFailed,
            internal::ReqwestError::RequestError => AuthError::OAuthFailed,
            internal::ReqwestError::AccessTokenError => AuthError::OAuthFailed,
        }
    }
}
