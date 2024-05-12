use crate::enums::errors::response::{self, Error, ResponseError};

pub enum SessionError {
    SessionNotFound,
    SessionAlreadyExists,
}

impl Error for SessionError {
    fn as_response(&self) -> ResponseError {
        match self {
            SessionError::SessionNotFound => {
                ResponseError::SessionError(response::SessionError::SessionNotFound)
            }
            SessionError::SessionAlreadyExists => {
                ResponseError::SessionError(response::SessionError::SessionAlreadyExists)
            }
        }
    }
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::SessionNotFound => write!(f, "Session not found"),
            SessionError::SessionAlreadyExists => write!(f, "Session already exists"),
        }
    }
}
