#[derive(Debug, Clone)]
pub enum Session {
    SessionNotFound,
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Session::SessionNotFound => write!(f, "Session not found"),
        }
    }
}
