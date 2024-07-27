use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<E> {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    pub error: E,
}

impl<E> IntoResponse for Response<E>
where
    E: Serialize,
{
    fn into_response(self) -> response::Response {
        let body = Json(json!({
            "message": self.message,
            "error": self.error,
        }));
        (self.status, body).into_response()
    }
}
