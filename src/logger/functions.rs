use super::{enums::LogLevel, types::{RequestLog, ResponseLog}, LOGGER};

pub fn info(message: &str, service: String) {
    for transport in LOGGER.transports.iter() {
        transport.log_raw(Some(message.to_string().clone()), service.clone(), LogLevel::INFO);
    }
}

pub fn error(error: &str, service: String) {
    for transport in LOGGER.transports.iter() {
        transport.log_raw(Some(error.to_string().clone()), service.clone(), LogLevel::ERROR);
    }
}

pub fn info_request(request: RequestLog) {
    for transport in LOGGER.transports.iter() {
        transport.log_request(request.clone());
    }
}

pub fn info_response(response: ResponseLog) {
    for transport in LOGGER.transports.iter() {
        transport.log_reponse(response.clone());
    }
}