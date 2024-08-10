use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}
