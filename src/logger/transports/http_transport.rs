use crate::{
    config::ENV,
    logger::{
        enums::LogLevel,
        types::{RawLogModel, RequestLog, RequestLogModel, ResponseLog, ResponseLogModel},
        Logger,
    },
};
use axum::{async_trait, http::StatusCode};

#[derive(Debug, Clone)]
pub struct HttpLogger {
    client: reqwest::Client,
}

#[async_trait]
impl Logger for HttpLogger {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn log_raw(&self, message: Option<String>, service: String, level: LogLevel) {
        let payload = RawLogModel::new(level, message, service);
        let _ = self
            .client
            .post(&ENV.logger_url)
            .json(&payload)
            .send()
            .await;
    }

    async fn log_request(&self, request: RequestLog) {
        let payload = RequestLogModel::new(
            RawLogModel::new(LogLevel::INFO, None, "request".to_string()),
            request.method,
            request.url,
            request.headers,
        );
        let _ = self
            .client
            .post(&ENV.logger_url)
            .json(&payload)
            .send()
            .await;
    }

    async fn log_reponse(&self, response: ResponseLog) {
        let payload = ResponseLogModel::new(
            RawLogModel::new(LogLevel::INFO, None, "response".to_string()),
            StatusCode::from_u16(response.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            response.headers,
        );
        let _ = self
            .client
            .post(&ENV.logger_url)
            .json(&payload)
            .send()
            .await;
    }
}
