use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
    Json,
};
use serde::Serialize;
use serde_json::json;

pub struct Response<D> {
    pub status: StatusCode,
    pub message: String,
    pub data: Option<D>,
}

impl<D: Serialize> IntoResponse for Response<D> {
    fn into_response(self) -> response::Response {
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

impl<D> Response<D> {
    pub fn new(status: StatusCode, message: &str) -> Self {
        Self {
            status,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn with_data(mut self, data: D) -> Self {
        self.data = Some(data);
        self
    }
}
