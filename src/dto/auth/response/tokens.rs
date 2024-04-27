use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}
