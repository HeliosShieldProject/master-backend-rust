use std::collections::HashMap;
use tokio::time::{Duration, Instant};

pub use agent::Agent;

use crate::{data::enums::Country, enums::errors::internal::Result};

use super::requests::login;

mod agent;
mod configs;
mod cookie;

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
            let new_cookie = login(self, country).await?;
            let mut cookie = agent.cookie.write().await;
            println!("Refreshed cookie: {:?}", new_cookie);
            cookie.cookie = new_cookie;
            cookie.expires_at = Instant::now() + Duration::from_secs(60 * 59);
        }

        Ok(cookie.cookie.clone())
    }
}
