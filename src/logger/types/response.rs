use axum::http::{HeaderMap, StatusCode};
use super::RawLogModel;

#[derive(Clone)]
pub struct ResponseLogModel {
    pub raw: RawLogModel,
    pub status: StatusCode,
    pub headers: HeaderMap,
}

#[derive(Clone)]
pub struct ResponseLog {
    pub status: u16,
    pub headers: HeaderMap,
}