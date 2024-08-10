use crate::{dto::response::error::Response, enums::errors::internal};
use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum Session {
    SessionNotFound,
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Session::SessionNotFound => write!(f, "SessionNotFound"),
        }
    }
}

impl IntoResponse for Session {
    fn into_response(self) -> response::Response {
        let (status, message) = match self {
            Session::SessionNotFound => (StatusCode::NOT_FOUND, "Session not found"),
        };

        Response {
            status,
            message: message.to_string(),
            error: self,
        }
        .into_response()
    }
}

impl From<internal::Session> for Session {
    fn from(error: internal::Session) -> Self {
        match error {
            internal::Session::SessionNotFound => Session::SessionNotFound,
        }
    }
}
