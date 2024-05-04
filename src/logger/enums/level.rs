use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum LogLevel {
    INFO,
    ERROR,
    DEBUG,
    WARN,
    TRACE,
}
