use std::collections::HashMap;

use super::RawLogModel;
use axum::http::HeaderMap;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct RequestLogModel {
    pub raw: RawLogModel,
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
}

impl RequestLogModel {
    pub fn new(raw: RawLogModel, url: String, method: String, headers: HeaderMap) -> Self {
        Self {
            raw,
            url,
            method,
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
pub struct RequestLogModelHttp {
    pub raw: RawLogModel,
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub source: String,
}

impl From<RequestLogModel> for RequestLogModelHttp {
    fn from(request: RequestLogModel) -> Self {
        Self {
            raw: request.raw,
            url: request.url,
            method: request.method,
            headers: request.headers,
            source: "master_backend".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct RequestLog {
    pub url: String,
    pub method: String,
    pub headers: HeaderMap,
}
