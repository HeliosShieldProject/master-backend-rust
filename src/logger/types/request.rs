use std::collections::HashMap;

use crate::logger::enums::LogLevel;

use super::RawLogModel;
use axum::http::HeaderMap;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct RequestLogModel {
    pub level: LogLevel,
    pub message: Option<String>,
    pub timestamp: NaiveDateTime,
    pub service: String,
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
}

impl RequestLogModel {
    pub fn new(raw: RawLogModel, url: String, method: String, headers: HeaderMap) -> Self {
        Self {
            level: raw.level,
            message: raw.message,
            timestamp: raw.timestamp,
            service: raw.service,
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
    pub level: LogLevel,
    pub message: Option<String>,
    pub timestamp: NaiveDateTime,
    pub service: String,
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub source: String,
}

impl From<RequestLogModel> for RequestLogModelHttp {
    fn from(request: RequestLogModel) -> Self {
        Self {
            level: request.level,
            message: request.message,
            timestamp: request.timestamp,
            service: request.service,            
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
