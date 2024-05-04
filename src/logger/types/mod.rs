mod request;
mod response;
use crate::logger::enums::LogLevel;
use chrono::NaiveDateTime;
pub use request::{RequestLog, RequestLogModel};
pub use response::{ResponseLog, ResponseLogModel};
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