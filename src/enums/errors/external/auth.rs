use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

use crate::{dto::response::error::Response, enums::errors::internal};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Auth {
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
}

impl std::fmt::Display for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Auth::WrongToken => write!(f, "WrongToken"),
            Auth::WrongPassword => write!(f, "WrongPassword"),
            Auth::WrongEmail => write!(f, "WrongEmail"),
            Auth::TokenCreation => write!(f, "TokenCreation"),
            Auth::UserNotFound => write!(f, "UserNotFound"),
            Auth::UserAlreadyExists => write!(f, "UserAlreadyExists"),
            Auth::PasswordIsSame => write!(f, "PasswordIsSame"),
            Auth::OAuthFailed => write!(f, "OAuthFailed"),
            Auth::OAuthDifferentEmail => write!(f, "OAuthDifferentEmail"),
            Auth::NoClassicAuth => write!(f, "NoClassicAuth"),
        }
    }
}

impl IntoResponse for Auth {
    fn into_response(self) -> response::Response {
        let (status, message) = match self {
            Auth::WrongToken => (StatusCode::UNAUTHORIZED, "Wrong token"),
            Auth::WrongPassword => (StatusCode::UNAUTHORIZED, "Wrong password"),
            Auth::WrongEmail => (StatusCode::UNAUTHORIZED, "Wrong email"),
            Auth::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            Auth::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            Auth::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            Auth::PasswordIsSame => (StatusCode::CONFLICT, "Password is the same"),
            Auth::OAuthFailed => (StatusCode::INTERNAL_SERVER_ERROR, "OAuth failed"),
            Auth::OAuthDifferentEmail => (StatusCode::UNAUTHORIZED, "OAuth different email"),
            Auth::NoClassicAuth => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "User has no classic auth",
            ),
        };

        Response {
            status,
            message: message.to_string(),
            error: self,
        }
        .into_response()
    }
}

impl From<internal::Auth> for Auth {
    fn from(error: internal::Auth) -> Self {
        match error {
            internal::Auth::WrongPassword => Auth::WrongPassword,
            internal::Auth::WrongEmail => Auth::WrongEmail,
            internal::Auth::TokenCreation => Auth::TokenCreation,
            internal::Auth::UserNotFound => Auth::UserNotFound,
            internal::Auth::UserAlreadyExists => Auth::UserAlreadyExists,
            internal::Auth::PasswordIsSame => Auth::PasswordIsSame,
            internal::Auth::OAuthFailed => Auth::OAuthFailed,
            internal::Auth::OAuthDifferentEmail => Auth::OAuthDifferentEmail,
            internal::Auth::NoClassicAuth => Auth::NoClassicAuth,
        }
    }
}

impl From<internal::Reqwest> for Auth {
    fn from(error: internal::Reqwest) -> Self {
        match error {
            internal::Reqwest::Json => Auth::OAuthFailed,
            internal::Reqwest::Request => Auth::OAuthFailed,
            internal::Reqwest::AccessToken => Auth::OAuthFailed,
        }
    }
}
