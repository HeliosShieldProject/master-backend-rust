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

#[derive(Clone)]
pub struct RequestLog {
    pub url: String,
    pub method: String,
    pub headers: HeaderMap,
}
