use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

pub struct SuccessResponse<T> {
    pub status: StatusCode,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> IntoResponse for SuccessResponse<T> {
    fn into_response(self) -> Response {
        let body = match self.data {
            Some(data) => Json(json!({
                "message": self.message,
                "data": data,
            })),
            None => Json(json!({
                "message": self.message,
            })),
        };
        (self.status, body).into_response()
    }
}

impl<T> SuccessResponse<T> {
    pub fn new(status: StatusCode, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
}
