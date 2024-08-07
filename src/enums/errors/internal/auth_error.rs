#[derive(Debug, Clone)]
pub enum AuthError {
    WrongToken,
    WrongPassword,
    WrongEmail,
    TokenCreation,
    UserNotFound,
    UserAlreadyExists,
    PasswordIsSame,
    OAuthFailed,
    OAuthDifferentEmail,
    NoClassicAuth,
    UnknownOAuthProvider,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::WrongToken => write!(f, "Wrong token"),
            AuthError::WrongPassword => write!(f, "Wrong password"),
            AuthError::WrongEmail => write!(f, "Wrong email"),
            AuthError::TokenCreation => write!(f, "Token creation error"),
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::UserAlreadyExists => write!(f, "User already exists"),
            AuthError::PasswordIsSame => write!(f, "Password is the same"),
            AuthError::OAuthFailed => write!(f, "OAuth failed"),
            AuthError::OAuthDifferentEmail => write!(f, "OAuth different email"),
            AuthError::NoClassicAuth => write!(f, "User has no classic auth"),
            AuthError::UnknownOAuthProvider => write!(f, "Unknown OAuth provider"),
        }
    }
}
