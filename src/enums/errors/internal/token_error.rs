use crate::enums::errors::{
    internal::{self, InternalError},
    response::{self, ResponseError},
};

pub enum TokenError {
    Encode,
}

impl internal::Error for jsonwebtoken::errors::Error {
    fn as_internal(&self) -> InternalError {
        InternalError::TokenError(TokenError::Encode)
    }
}

impl response::Error for TokenError {
    fn as_response(&self) -> ResponseError {
        match self {
            TokenError::Encode => ResponseError::AuthError(response::AuthError::WrongToken),
        }
    }
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::Encode => write!(f, "Token encoding error"),
        }
    }
}