use std::collections::HashMap;

use super::RawLogModel;
use axum::http::{HeaderMap, StatusCode};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ResponseLogModel {
    pub raw: RawLogModel,
    pub status: String,
    pub headers: HashMap<String, String>,
}

impl ResponseLogModel {
    pub fn new(raw: RawLogModel, status: StatusCode, headers: HeaderMap) -> Self {
        Self {
            raw,
            status: status.to_string(),
            headers: headers
                .iter()
                .map(|(key, value)| {
                    (
                        key.as_str().to_string(),
                        value.to_str().unwrap().to_string(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct ResponseLogModelHttp {
    pub raw: RawLogModel,
    pub status: String,
    pub headers: HashMap<String, String>,
    pub source: String,
}

impl From<ResponseLogModel> for ResponseLogModelHttp {
    fn from(response: ResponseLogModel) -> Self {
        Self {
            raw: response.raw,
            status: response.status,
            headers: response.headers,
            source: "master_backend".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct ResponseLog {
    pub status: u16,
    pub headers: HeaderMap,
}
