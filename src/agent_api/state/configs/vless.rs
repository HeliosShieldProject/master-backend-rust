use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Vless {
    pub inbound_id: u32,
    pub port: u16,
}
