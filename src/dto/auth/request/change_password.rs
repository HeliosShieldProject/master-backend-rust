use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub password: String,
}
