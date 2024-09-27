use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;

use crate::{data::enums::Country, enums::errors::internal::Result};

use super::requests::login;

#[derive(Clone)]
pub struct CookieInfo {
    pub cookie: String,
    pub expires_at: Instant,
}

#[derive(Clone)]
pub struct ShadowsocksConfig {
    pub port: u16,
    pub method: String,
    pub password: String,
}

#[derive(Clone)]
pub struct VlessConfig {
    pub port: u16,
}

#[derive(Clone)]
pub struct Agent {
    pub host: String,
    pub secure_path: String,
    pub username: String,
    pub password: String,
    pub shadowsocks_config: ShadowsocksConfig,
    pub vless_config: VlessConfig,
    pub cookie: Arc<RwLock<CookieInfo>>,
}

#[derive(Clone)]
pub struct AgentState {
    pub client: reqwest::Client,
    pub agents: HashMap<Country, Agent>,
}

impl AgentState {
    pub fn new(servers: HashMap<Country, Agent>) -> Self {
        AgentState {
            agents: servers,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_or_refresh_cookie(&self, country: &Country) -> Result<String> {
        let agent = self.agents.get(country).unwrap();
        let cookie = agent.cookie.read().await;

        if cookie.expires_at < Instant::now() {
            let new_cookie = login(
                &self.client,
                &agent.secure_path,
                &agent.username,
                &agent.password,
            )
            .await?;

            let mut cookie = agent.cookie.write().await;
            cookie.cookie = new_cookie;
            cookie.expires_at = Instant::now() + Duration::from_secs(60 * 59);
        }

        Ok(cookie.cookie.clone())
    }
}
