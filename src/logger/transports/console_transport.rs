use crate::logger::{
    enums::LogLevel,
    types::{RawLogModel, RequestLog, RequestLogModel, ResponseLog, ResponseLogModel},
    Logger,
};
use axum::{async_trait, http::StatusCode};
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct ConsoleLogger {}

#[async_trait]
impl Logger for ConsoleLogger {
    fn new() -> Self {
        Self {}
    }

    async fn log_raw(&self, message: Option<String>, service: String, level: LogLevel) {
        let message = RawLogModel::new(level, message, service).format();
        match level {
            LogLevel::ERROR => eprintln!("{}", message),
            _ => println!("{}", message),
        }
    }

    async fn log_request(&self, request: RequestLog) {
        let message = RequestLogModel::new(
            RawLogModel::new(LogLevel::INFO, None, "request".to_string()),
            request.method,
            request.url,
            request.headers,
        )
        .format();
        println!("{}", message);
    }

    async fn log_reponse(&self, response: ResponseLog) {
        let message = ResponseLogModel::new(
            RawLogModel::new(LogLevel::INFO, None, "response".to_string()),
            StatusCode::from_u16(response.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            response.headers,
        )
        .format();
        println!("{}", message);
    }
}

fn format_service(service: &str) -> String {
    format!("{: <14}", service.to_uppercase())
}

trait Format {
    fn format(&self) -> String;
}

impl Format for RawLogModel {
    fn format(&self) -> String {
        format!(
            "[{}] [{}] [{}] - {}",
            self.timestamp.time(),
            self.level.to_color(),
            format_service(&self.service).blue(),
            self.message.as_ref().unwrap_or(&String::new())
        )
    }
}

impl Format for RequestLogModel {
    fn format(&self) -> String {
        let method = match self.method.as_str() {
            "GET" => self.method.green(),
            "POST" => self.method.yellow(),
            "PUT" => self.method.blue(),
            "DELETE" => self.method.red(),
            _ => self.method.purple(),
        };
        format!(
            "[{}] [{}] [{}] - {} {} {:?}",
            self.timestamp.time(),
            self.level.to_color(),
            format_service(&self.service).purple(),
            method,
            self.url,
            self.headers
        )
    }
}

impl Format for ResponseLogModel {
    fn format(&self) -> String {
        let status = match self
            .status
            .split_whitespace()
            .next()
            .unwrap()
            .parse::<u16>()
            .unwrap()
        {
            200..=299 => self.status.to_string().green(),
            300..=399 => self.status.to_string().yellow(),
            400..=499 => self.status.to_string().bright_red(),
            500..=599 => self.status.to_string().red(),
            _ => self.status.to_string().purple(),
        };
        format!(
            "[{}] [{}] [{}] - {} {:?}",
            self.timestamp.time(),
            self.level.to_color(),
            format_service(&self.service).purple(),
            status,
            self.headers
        )
    }
}

pub trait Color {
    fn to_color(self) -> colored::ColoredString;
}

impl Color for LogLevel {
    fn to_color(self) -> colored::ColoredString {
        match self {
            LogLevel::INFO => "INFO ".green(),
            LogLevel::ERROR => "ERROR".red(),
            LogLevel::DEBUG => "DEBUG".blue(),
            LogLevel::WARN => "WARN ".yellow(),
            LogLevel::TRACE => "TRACE".purple(),
        }
    }
}
