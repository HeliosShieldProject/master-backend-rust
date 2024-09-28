use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Shadowsocks {
    pub inbound_id: u32,
    pub port: u16,
    pub encryption: String,
    pub password: String,
}
