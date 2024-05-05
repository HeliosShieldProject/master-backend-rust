mod request;
mod response;
use crate::logger::enums::LogLevel;
use chrono::NaiveDateTime;
pub use request::{RequestLog, RequestLogModel, RequestLogModelHttp};
pub use response::{ResponseLog, ResponseLogModel, ResponseLogModelHttp};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct RawLogModel {
    pub level: LogLevel,
    pub message: Option<String>,
    pub timestamp: NaiveDateTime,
    pub service: String,
}

impl RawLogModel {
    pub fn new(level: LogLevel, message: Option<String>, service: String) -> Self {
        Self {
            level,
            message,
            timestamp: chrono::Utc::now().naive_utc(),
            service,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct RawLogModelHttp {
    pub level: LogLevel,
    pub message: Option<String>,
    pub timestamp: NaiveDateTime,
    pub service: String,
    pub source: String,
}

impl From<RawLogModel> for RawLogModelHttp {
    fn from(raw: RawLogModel) -> Self {
        Self {
            level: raw.level,
            message: raw.message,
            timestamp: raw.timestamp,
            service: raw.service,
            source: "master_backend".to_string(),
        }
    }
}