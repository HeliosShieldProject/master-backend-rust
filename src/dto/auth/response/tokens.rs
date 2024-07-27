use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}
