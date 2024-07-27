use crate::dto::response::error::Response;
use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum CountryError {
    CountryNotFound,
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
