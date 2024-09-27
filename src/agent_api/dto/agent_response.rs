use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentResponse<T> {
    pub success: bool,
    pub msg: String,
    pub obj: Option<T>,
}
