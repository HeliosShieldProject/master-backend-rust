use crate::enums::oauth_providers::OAuthProvider;

#[derive(Clone, Debug)]
pub struct OAuthCode {
    pub code: String,
    pub provider: OAuthProvider,
}
