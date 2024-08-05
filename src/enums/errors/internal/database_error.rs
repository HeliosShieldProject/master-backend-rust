use super::InternalError;

#[derive(Debug)]
pub enum DatabaseError {
    Pool,
    Interact,
    Diesel,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::Pool => write!(f, "Pool error"),
            DatabaseError::Interact => write!(f, "Interact error"),
            DatabaseError::Diesel => write!(f, "Diesel error"),
        }
    }
}

impl From<deadpool_diesel::PoolError> for InternalError {
    fn from(error: deadpool_diesel::PoolError) -> Self {
        tracing::error!("{}", error);
        InternalError::DatabaseError(DatabaseError::Pool)
    }
}

impl From<deadpool_diesel::InteractError> for InternalError {
    fn from(error: deadpool_diesel::InteractError) -> Self {
        tracing::error!("{}", error);
        InternalError::DatabaseError(DatabaseError::Interact)
    }
}

impl From<diesel::result::Error> for InternalError {
    fn from(error: diesel::result::Error) -> Self {
        tracing::error!("{}", error);
        InternalError::DatabaseError(DatabaseError::Diesel)
    }
}
