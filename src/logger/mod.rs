use self::{
    enums::{Handlers, LogLevel},
    transports::{ConsoleLogger, HttpLogger},
    types::{RequestLog, ResponseLog},
};
use crate::{
    config::ENV,
    enums::errors::{internal::InternalError, response::ResponseError},
};
use axum::async_trait;
use once_cell::sync::Lazy;

pub mod enums;
mod functions;
mod transports;
pub mod types;

use enums::Services;
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

pub static LOGGER: Lazy<LoggerConfig> = Lazy::new(|| match ENV.rust_env.as_str() {
    "development" => LoggerConfig {
        transports: vec![Box::new(ConsoleLogger::new()), Box::new(HttpLogger::new())],
    },
    "production" => LoggerConfig {
        transports: vec![Box::new(ConsoleLogger::new()), Box::new(HttpLogger::new())],
    },
    _ => LoggerConfig { transports: vec![] },
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
}

pub trait ResultExt<R> {
    async fn log_error(self, service: Services) -> Result<R, InternalError>;
    async fn log<T: AsRef<str>>(self, message: T, service: Services) -> Result<R, InternalError>;
}

impl<R> ResultExt<R> for Result<R, InternalError> {
    async fn log_error(self, service: Services) -> Result<R, InternalError> {
        if let Err(e) = &self {
            error(e.to_string().as_str(), service.to_string()).await;
        }
        self
    }

    async fn log<T: AsRef<str>>(self, message: T, service: Services) -> Result<R, InternalError> {
        match self {
            Ok(_) => info(message.as_ref(), service.to_string()).await,
            Err(ref e) => error(e.to_string().as_str(), service.to_string()).await,
        }
        self
    }
}

pub trait ResultExtReponse<R> {
    async fn log_error(self, handler: Handlers) -> Result<R, ResponseError>;
    async fn log<T: AsRef<str>>(self, handler: T, handler: Handlers) -> Result<R, ResponseError>;
}

impl<R> ResultExtReponse<R> for Result<R, ResponseError> {
    async fn log_error(self, handler: Handlers) -> Result<R, ResponseError> {
        if let Err(e) = &self {
            error(e.to_string().as_str(), handler.to_string()).await;
        }
        self
    }

    async fn log<T: AsRef<str>>(self, message: T, handler: Handlers) -> Result<R, ResponseError> {
        match self {
            Ok(_) => info(message.as_ref(), handler.to_string()).await,
            Err(ref e) => error(e.to_string().as_str(), handler.to_string()).await,
        }
        self
    }
}
