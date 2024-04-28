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
