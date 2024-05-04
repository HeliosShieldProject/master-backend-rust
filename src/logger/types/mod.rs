mod request;
mod response;
use crate::logger::enums::LogLevel;
use chrono::NaiveDateTime;
pub use request::{RequestLog, RequestLogModel};
pub use response::{ResponseLog, ResponseLogModel};

#[derive(Clone)]
pub struct RawLogModel {
    pub level: LogLevel,
    pub message: Option<String>,
    pub timestamp: NaiveDateTime,
    pub service: String,
}
