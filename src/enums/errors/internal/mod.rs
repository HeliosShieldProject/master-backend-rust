mod hash_error;
pub use hash_error::HashError;

mod token_error;
pub use token_error::TokenError;

mod auth_error;
pub use auth_error::AuthError;

mod device_error;
pub use device_error::DeviceError;

mod country_error;
pub use country_error::CountryError;

mod session_error;
pub use session_error::SessionError;

mod database_error;
pub use database_error::DatabaseError;

mod reqwest_error;
pub use reqwest_error::ReqwestError;

#[derive(Debug, Clone)]
pub enum InternalError {
    HashError(HashError),
    TokenError(TokenError),
    AuthError(AuthError),
    DeviceError(DeviceError),
    CountryError(CountryError),
    SessionError(SessionError),
    DatabaseError(DatabaseError),
    ReqwestError(ReqwestError),
    SerializationError,
    UuidParse,
    Internal,
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalError::DatabaseError(e) => write!(f, "{}", e),
            InternalError::HashError(e) => write!(f, "{}", e),
            InternalError::TokenError(e) => write!(f, "{}", e),
            InternalError::AuthError(e) => write!(f, "{}", e),
            InternalError::DeviceError(e) => write!(f, "{}", e),
            InternalError::CountryError(e) => write!(f, "{}", e),
            InternalError::SessionError(e) => write!(f, "{}", e),
            InternalError::ReqwestError(e) => write!(f, "{}", e),
            InternalError::SerializationError => write!(f, "Serialization error"),
            InternalError::UuidParse => write!(f, "Uuid parse error"),
            InternalError::Internal => write!(f, "Internal error"),
        }
    }
}

pub type Result<T> = core::result::Result<T, InternalError>;
