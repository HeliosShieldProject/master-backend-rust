mod hash_error;
pub use hash_error::HashError;

mod token_error;
pub use token_error::TokenError;

pub mod auth_error;
pub use auth_error::AuthError;

pub mod device_error;
pub use device_error::DeviceError;

pub mod country_error;
pub use country_error::CountryError;

pub mod session_error;
pub use session_error::SessionError;

pub enum InternalError {
    HashError(HashError),
    TokenError(TokenError),
    AuthError(AuthError),
    DeviceError(DeviceError),
    CountryError(CountryError),
    SessionError(SessionError),
    UuidParse,
    Internal,
}

pub trait Error {
    fn as_internal(&self) -> InternalError;
}

pub fn to_internal<T: Error>(error: T) -> InternalError {
    error.as_internal()
}

impl Error for deadpool_diesel::PoolError {
    fn as_internal(&self) -> InternalError {
        InternalError::Internal
    }
}

impl Error for deadpool_diesel::InteractError {
    fn as_internal(&self) -> InternalError {
        InternalError::Internal
    }
}

impl Error for uuid::Error {
    fn as_internal(&self) -> InternalError {
        InternalError::UuidParse
    }
}
