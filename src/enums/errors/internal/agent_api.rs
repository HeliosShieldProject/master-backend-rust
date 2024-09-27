#[derive(Debug, Clone)]
pub enum AgentAPI {
    Internal,
    LoginFailed,
}

impl std::fmt::Display for AgentAPI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentAPI::LoginFailed => write!(f, "Login failed"),
            AgentAPI::Internal => write!(f, "Internal error"),
        }
    }
}