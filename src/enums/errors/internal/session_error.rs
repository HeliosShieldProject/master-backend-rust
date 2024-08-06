#[derive(Debug, Clone)]
pub enum SessionError {
    SessionNotFound,
    SessionAlreadyExists,
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::SessionNotFound => write!(f, "Session not found"),
            SessionError::SessionAlreadyExists => write!(f, "Session already exists"),
        }
    }
}
