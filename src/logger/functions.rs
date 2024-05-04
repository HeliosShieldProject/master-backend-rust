use super::{enums::LogLevel, types::{RequestLog, ResponseLog}, LOGGER};

pub async fn info(message: &str, service: String) {
    for transport in LOGGER.transports.iter() {
        transport.log_raw(Some(message.to_string().clone()), service.clone(), LogLevel::INFO).await;
    }
}

pub async fn error(error: &str, service: String) {
    for transport in LOGGER.transports.iter() {
        transport.log_raw(Some(error.to_string().clone()), service.clone(), LogLevel::ERROR).await;
    }
}

pub async fn info_request(request: RequestLog) {
    for transport in LOGGER.transports.iter() {
        transport.log_request(request.clone()).await;
    }
}

pub async fn info_response(response: ResponseLog) {
    for transport in LOGGER.transports.iter() {
        transport.log_reponse(response.clone()).await;
    }
}