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

impl SessionError {
    pub fn to_string(&self) -> String {
        match self {
            SessionError::SessionNotFound => "Session not found".to_string(),
            SessionError::SessionAlreadyExists => "Session already exists".to_string(),
        }
    }
}