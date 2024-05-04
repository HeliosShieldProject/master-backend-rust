use self::{
    enums::LogLevel,
    transports::{ConsoleLogger, HttpLogger},
    types::{RequestLog, ResponseLog},
};
use crate::{config::ENV, services::Services};
use axum::async_trait;
use once_cell::sync::Lazy;

pub mod enums;
mod functions;
mod transports;
pub mod types;

pub use functions::{error, info, info_request, info_response};

#[async_trait]
pub trait Logger {
    fn new() -> Self
    where
        Self: Sized;
    async fn log_raw(&self, message: Option<String>, service: String, level: LogLevel);
    async fn log_request(&self, request: RequestLog);
    async fn log_reponse(&self, response: ResponseLog);
}

pub struct LoggerConfig {
    pub transports: Vec<Box<dyn Logger + Sync + Send>>,
}

pub static LOGGER: Lazy<LoggerConfig> = Lazy::new(|| {
    match ENV.rust_env.as_str() {
        "development" => LoggerConfig {
            transports: vec![Box::new(ConsoleLogger::new()), Box::new(HttpLogger::new())],
        },
        // "production" => LoggerConfig {
        //     transports: vec![Box::new(FileLogger {}), Box::new(MongoLogger {})],
        // },
        _ => LoggerConfig { transports: vec![] },
    }
});

pub struct ContextLogger {
    pub service: Services,
}

impl ContextLogger {
    pub const fn new(service: Services) -> Self {
        Self { service }
    }

    pub async fn info<T: AsRef<str>>(&self, message: T) {
        info(message.as_ref(), self.service.to_string()).await;
    }

    pub async fn error<T: AsRef<str>>(&self, message: T) {
        error(message.as_ref(), self.service.to_string()).await;
    }
}
