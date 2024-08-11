use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfirmEmailQuery {
    pub token: Uuid,
}
