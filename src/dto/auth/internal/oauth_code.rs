use crate::data::enums::OAuthProvider;

#[derive(Clone, Debug)]
pub struct OAuthCode {
    pub code: String,
    pub provider: OAuthProvider,
}
