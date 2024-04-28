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
