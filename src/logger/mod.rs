use self::{
    enums::LogLevel,
    transports::ConsoleLogger,
    types::{RequestLog, ResponseLog},
};
use crate::{config::ENV, services::Services};
use once_cell::sync::Lazy;

pub mod enums;
mod functions;
mod transports;
pub mod types;

pub use functions::{info, info_request, info_response, error};

pub trait Logger {
    fn log_raw(&self, message: Option<String>, service: String, level: LogLevel);
    fn log_request(&self, request: RequestLog);
    fn log_reponse(&self, response: ResponseLog);
}

pub struct LoggerConfig {
    pub transports: Vec<Box<dyn Logger + Sync + Send>>,
}

pub static LOGGER: Lazy<LoggerConfig> = Lazy::new(|| {
    match ENV.rust_env.as_str() {
        "development" => LoggerConfig {
            transports: vec![Box::new(ConsoleLogger {})],
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

    pub fn info<T: AsRef<str>>(&self, message: T) {
        info(message.as_ref(), self.service.to_string());
    }

    pub fn error<T: AsRef<str>>(&self, message: T) {
        error(message.as_ref(), self.service.to_string());
    }
}