use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct OAuthUser {
    pub email: String,
    pub metadata: serde_json::Value,
}
