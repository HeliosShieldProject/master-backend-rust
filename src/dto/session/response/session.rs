use crate::data::models;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub session_id: Uuid,
    pub server_public_key: String,
    pub wireguard_uri: String,
    pub user_ip: String,
    pub user_private_key: String,
}

impl Session {
    pub fn new(session: models::Session, server: models::Server, config: models::Config) -> Self {
        Self {
            session_id: session.id,
            server_public_key: server.public_key,
            wireguard_uri: server.wireguard_uri,
            user_ip: config.user_ip,
            user_private_key: config.private_key,
        }
    }
}
