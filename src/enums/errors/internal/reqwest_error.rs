use super::InternalError;

#[derive(Debug, Clone)]
pub enum ReqwestError {
    RequestError,
    JsonError,
    AccessTokenError,
}

impl std::fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReqwestError::RequestError => write!(f, "Request error"),
            ReqwestError::JsonError => write!(f, "Json error"),
            ReqwestError::AccessTokenError => write!(f, "Access token error"),
        }
    }
}

impl From<reqwest::Error> for InternalError {
    fn from(error: reqwest::Error) -> Self {
        tracing::error!("{}", error);
        InternalError::ReqwestError(ReqwestError::RequestError)
    }
}

impl From<serde_json::Error> for InternalError {
    fn from(error: serde_json::Error) -> Self {
        tracing::error!("{}", error);
        InternalError::ReqwestError(ReqwestError::JsonError)
    }
}
