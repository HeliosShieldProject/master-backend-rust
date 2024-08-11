use super::Error;

#[derive(Debug, Clone)]
pub enum Resend {
    SendConfirmation,
}

impl std::fmt::Display for Resend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resend::SendConfirmation => write!(f, "Send confirmation error"),
        }
    }
}

impl From<resend_rs::Error> for Error {
    fn from(error: resend_rs::Error) -> Self {
        tracing::error!("{}", error);
        Error::Resend(Resend::SendConfirmation)
    }
}
