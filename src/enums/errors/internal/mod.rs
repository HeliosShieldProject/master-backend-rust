mod agent_api;
mod auth;
mod database;
mod device;
mod hash;
mod reqwest;
mod resend;
mod session;
mod token;

pub use agent_api::AgentAPI;
pub use auth::Auth;
pub use database::Database;
pub use device::Device;
pub use hash::Hash;
pub use reqwest::Reqwest;
pub use resend::Resend;
pub use session::Session;
pub use token::Token;

#[derive(Debug, Clone)]
pub enum Error {
    Hash(Hash),
    Token(Token),
    Auth(Auth),
    Session(Session),
    Database(Database),
    Reqwest(Reqwest),
    Resend(Resend),
    Device(Device),
    AgentAPI(AgentAPI),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Database(e) => write!(f, "{}", e),
            Error::Hash(e) => write!(f, "{}", e),
            Error::Token(e) => write!(f, "{}", e),
            Error::Auth(e) => write!(f, "{}", e),
            Error::Session(e) => write!(f, "{}", e),
            Error::Reqwest(e) => write!(f, "{}", e),
            Error::Resend(e) => write!(f, "{}", e),
            Error::Device(e) => write!(f, "{}", e),
            Error::AgentAPI(e) => write!(f, "{}", e),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
