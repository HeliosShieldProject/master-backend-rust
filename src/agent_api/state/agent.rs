use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::{sync::RwLock, time::Instant};

use super::{configs, cookie::Cookie};

#[derive(Clone)]
pub struct Agent {
    pub host: String,
    pub port: u16,
    pub secure_path: String,
    pub username: String,
    pub password: String,
    pub shadowsocks_config: configs::Shadowsocks,
    pub vless_config: configs::Vless,
    pub cookie: Arc<RwLock<Cookie>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct AgentBuilder {
    host: String,
    port: u16,
    secure_path: String,
    username: String,
    password: String,
    shadowsocks_config: configs::Shadowsocks,
    vless_config: configs::Vless,
}

impl From<AgentBuilder> for Agent {
    fn from(builder: AgentBuilder) -> Self {
        Agent {
            host: builder.host,
            port: builder.port,
            secure_path: builder.secure_path,
            username: builder.username,
            password: builder.password,
            shadowsocks_config: builder.shadowsocks_config,
            vless_config: builder.vless_config,
            cookie: Arc::new(RwLock::new(Cookie {
                cookie: String::new(),
                expires_at: Instant::now(),
            })),
        }
    }
}

impl From<&str> for Agent {
    fn from(s: &str) -> Self {
        let builder: AgentBuilder = serde_json::from_str(s).expect("Agent config is invalid");
        builder.into()
    }
}
