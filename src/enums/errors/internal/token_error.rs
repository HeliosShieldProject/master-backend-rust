use super::InternalError;

pub enum TokenError {
    Encode,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::Encode => write!(f, "Token encoding error"),
        }
    }
}

impl std::convert::From<uuid::Error> for InternalError {
    fn from(error: uuid::Error) -> Self {
        tracing::error!("{}", error);
        InternalError::TokenError(TokenError::Encode)
    }
}

impl From<jsonwebtoken::errors::Error> for InternalError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        tracing::error!("{}", error);
        InternalError::TokenError(TokenError::Encode)
    }
}
