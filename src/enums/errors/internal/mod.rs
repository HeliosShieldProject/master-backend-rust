mod auth;
mod database;
mod hash;
mod reqwest;
mod session;
mod token;

pub use auth::Auth;
pub use database::Database;
pub use hash::Hash;
pub use reqwest::Reqwest;
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
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
