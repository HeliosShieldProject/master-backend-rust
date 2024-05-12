use crate::enums::errors::response::{self, Error, ResponseError};

pub enum AuthError {
    WrongToken,
    WrongPassword,
    WrongEmail,
    MissingCredentials,
    MissingDevice,
    TokenCreation,
    UserNotFound,
    UserAlreadyExists,
    PasswordIsSame,
}

impl Error for AuthError {
    fn as_response(&self) -> ResponseError {
        match self {
            AuthError::WrongToken => ResponseError::AuthError(response::AuthError::WrongToken),
            AuthError::WrongPassword => {
                ResponseError::AuthError(response::AuthError::WrongPassword)
            }
            AuthError::WrongEmail => ResponseError::AuthError(response::AuthError::WrongEmail),
            AuthError::MissingCredentials => {
                ResponseError::AuthError(response::AuthError::MissingCredentials)
            }
            AuthError::MissingDevice => {
                ResponseError::AuthError(response::AuthError::MissingDevice)
            }
            AuthError::TokenCreation => {
                ResponseError::AuthError(response::AuthError::TokenCreation)
            }
            AuthError::UserNotFound => ResponseError::AuthError(response::AuthError::UserNotFound),
            AuthError::UserAlreadyExists => {
                ResponseError::AuthError(response::AuthError::UserAlreadyExists)
            }
            AuthError::PasswordIsSame => {
                ResponseError::AuthError(response::AuthError::PasswordIsSame)
            }
        }
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::WrongToken => write!(f, "Wrong token"),
            AuthError::WrongPassword => write!(f, "Wrong password"),
            AuthError::WrongEmail => write!(f, "Wrong email"),
            AuthError::MissingCredentials => write!(f, "Missing credentials"),
            AuthError::MissingDevice => write!(f, "Missing device"),
            AuthError::TokenCreation => write!(f, "Token creation error"),
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::UserAlreadyExists => write!(f, "User already exists"),
            AuthError::PasswordIsSame => write!(f, "Password is the same"),
        }
    }
}
