use crate::dto::response::error::ErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]

pub enum CountryError {
    CountryNotFound,
}

impl IntoResponse for CountryError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            CountryError::CountryNotFound => (StatusCode::NOT_FOUND, "Country not found"),
        };

        ErrorResponse {
            status,
            message: message.to_string(),
            error: self,
        }
        .into_response()
    }
}
