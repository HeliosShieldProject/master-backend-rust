use super::Error;

#[derive(Debug, Clone)]
pub enum Database {
    Pool,
    Interact,
    Diesel,
}

impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Database::Pool => write!(f, "Pool error"),
            Database::Interact => write!(f, "Interact error"),
            Database::Diesel => write!(f, "Diesel error"),
        }
    }
}

impl From<deadpool_diesel::PoolError> for Error {
    fn from(error: deadpool_diesel::PoolError) -> Self {
        tracing::error!("{}", error);
        Error::Database(Database::Pool)
    }
}

impl From<deadpool_diesel::InteractError> for Error {
    fn from(error: deadpool_diesel::InteractError) -> Self {
        tracing::error!("{}", error);
        Error::Database(Database::Interact)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        tracing::error!("{}", error);
        Error::Database(Database::Diesel)
    }
}
