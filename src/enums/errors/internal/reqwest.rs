use super::Error;

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum Reqwest {
    Request,
    Json,
    AccessToken,
}

impl std::fmt::Display for Reqwest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reqwest::Request => write!(f, "Request error"),
            Reqwest::Json => write!(f, "Json error"),
            Reqwest::AccessToken => write!(f, "Access token error"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        tracing::error!("{}", error);
        Error::Reqwest(Reqwest::Request)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        tracing::error!("{}", error);
        Error::Reqwest(Reqwest::Json)
    }
}
