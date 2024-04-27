use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub exp: i64,
    pub iat: i64,
    pub device_id: Uuid,
    pub user_id: Uuid,
}
