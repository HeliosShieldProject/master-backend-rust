use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

pub struct ErrorResponse<T> {
    pub status: StatusCode,
    pub message: String,
    pub error: T,
}

impl<T> IntoResponse for ErrorResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = Json(json!({
            "message": self.message,
            "error": self.error,
        }));
        (self.status, body).into_response()
    }
}
