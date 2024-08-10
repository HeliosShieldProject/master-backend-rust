use super::Error;

#[derive(Debug, Clone)]
pub enum Token {
    Encode,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Encode => write!(f, "Token encoding error"),
        }
    }
}

impl std::convert::From<uuid::Error> for Error {
    fn from(error: uuid::Error) -> Self {
        tracing::error!("{}", error);
        Error::Token(Token::Encode)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        tracing::error!("{}", error);
        Error::Token(Token::Encode)
    }
}
