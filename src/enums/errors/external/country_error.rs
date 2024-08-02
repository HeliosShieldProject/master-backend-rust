use crate::{dto::response::error::Response, enums::errors::internal};
use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum CountryError {
    CountryNotFound,
}

impl std::fmt::Display for CountryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CountryError::CountryNotFound => write!(f, "CountryNotFound"),
        }
    }
}

impl IntoResponse for CountryError {
    fn into_response(self) -> response::Response {
        let (status, message) = match self {
            CountryError::CountryNotFound => (StatusCode::NOT_FOUND, "Country not found"),
        };

        Response {
            status,
            message: message.to_string(),
            error: self,
        }
        .into_response()
    }
}

impl From<internal::CountryError> for CountryError {
    fn from(error: internal::CountryError) -> Self {
        match error {
            internal::CountryError::CountryNotFound => CountryError::CountryNotFound,
        }
    }
}
