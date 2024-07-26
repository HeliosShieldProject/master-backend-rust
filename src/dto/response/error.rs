use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
    Json,
};
use serde::Serialize;
use serde_json::json;

pub struct Response<E> {
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
