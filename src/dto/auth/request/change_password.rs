use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ChangePasswordRequest {
    #[schema(example = "strong_password")]
    pub password: String,
}
