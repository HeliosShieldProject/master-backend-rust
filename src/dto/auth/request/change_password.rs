use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangePasswordRequest {
    pub password: String,
}
