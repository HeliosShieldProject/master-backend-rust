use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OAuthProvider {
    Google,
    Github,
    Discord,
}
