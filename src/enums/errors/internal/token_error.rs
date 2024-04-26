use super::{Error, InternalError};

pub enum TokenError {
    Encode,
}

impl Error for jsonwebtoken::errors::Error {
    fn as_internal(&self) -> InternalError {
        InternalError::TokenError(TokenError::Encode)
    }
}
