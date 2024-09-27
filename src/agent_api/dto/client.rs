use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub inbound_id: u32,
    pub link: String,
}
