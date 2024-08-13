#[derive(Debug, Clone)]
pub enum Session {
    NotFound,
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Session::NotFound => write!(f, "Session not found"),
        }
    }
}
