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

impl TokenError {
    pub fn to_string(&self) -> String {
        match self {
            TokenError::Encode => "Token encode error".to_string(),
        }
    }
}