use crate::enums::errors::response::{self, Error, ResponseError};

pub enum HashError {
    Hash,
    Verify,
}

impl Error for HashError {
    fn as_response(&self) -> ResponseError {
        match self {
            HashError::Hash => ResponseError::Internal,
            HashError::Verify => ResponseError::AuthError(response::AuthError::WrongPassword),
        }
    }
}

impl std::fmt::Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HashError::Hash => write!(f, "Hash error"),
            HashError::Verify => write!(f, "Verify error"),
        }
    }
}