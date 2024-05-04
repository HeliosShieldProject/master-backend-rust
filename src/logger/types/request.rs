use super::RawLogModel;
use axum::http::HeaderMap;

#[derive(Clone)]
pub struct RequestLogModel {
    pub raw: RawLogModel,
    pub url: String,
    pub method: String,
    pub headers: HeaderMap,
}

#[derive(Clone)]
pub struct RequestLog {
    pub url: String,
    pub method: String,
    pub headers: HeaderMap,
}
