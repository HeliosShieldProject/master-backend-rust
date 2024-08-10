#[derive(Debug, Clone)]
pub enum Auth {
    WrongPassword,
    WrongEmail,
    TokenCreation,
    UserNotFound,
    UserAlreadyExists,
    PasswordIsSame,
    OAuthFailed,
    OAuthDifferentEmail,
    NoClassicAuth,
}

impl std::fmt::Display for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Auth::WrongPassword => write!(f, "Wrong password"),
            Auth::WrongEmail => write!(f, "Wrong email"),
            Auth::TokenCreation => write!(f, "Token creation error"),
            Auth::UserNotFound => write!(f, "User not found"),
            Auth::UserAlreadyExists => write!(f, "User already exists"),
            Auth::PasswordIsSame => write!(f, "Password is the same"),
            Auth::OAuthFailed => write!(f, "OAuth failed"),
            Auth::OAuthDifferentEmail => write!(f, "OAuth different email"),
            Auth::NoClassicAuth => write!(f, "User has no classic auth"),
        }
    }
}