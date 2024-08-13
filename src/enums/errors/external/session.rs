use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

use crate::{dto::response::error::Response, enums::errors::internal};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum Session {
    NotFound,
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Session::NotFound => write!(f, "SessionNotFound"),
        }
    }
}

impl IntoResponse for Session {
    fn into_response(self) -> response::Response {
        let (status, message) = match self {
            Session::NotFound => (StatusCode::NOT_FOUND, "Session not found"),
        };

        Response {
            status,
            message: message.to_string(),
            error: self.to_string(),
        }
        .into_response()
    }
}

impl From<internal::Session> for Session {
    fn from(error: internal::Session) -> Self {
        match error {
            internal::Session::NotFound => Session::NotFound,
        }
    }
}
