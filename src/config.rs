use std::env;

use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerMode {
    Development,
    Production,
}

impl std::str::FromStr for ServerMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "development" => Ok(ServerMode::Development),
            "production" => Ok(ServerMode::Production),
            _ => Err("Unknown server mode".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub master_backend_url: String,
    pub database_url: String,
    pub master_metrics_url: String,
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub discord_client_secret: String,
    pub discord_client_id: String,
    pub github_client_secret: String,
    pub github_client_id: String,
    pub google_client_secret: String,
    pub google_client_id: String,
    pub resend_api_key: String,
    pub server_mode: ServerMode,
    pub agent_config_uk: String,
}

pub static ENV: Lazy<Config> = Lazy::new(|| {
    load_env();
    Config {
        master_backend_url: env::var("MASTER_BACKEND_URL").expect("MASTER_BACKEND_URL must be set"),
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        master_metrics_url: env::var("MASTER_METRICS_URL").expect("MASTER_METRICS_URL must be set"),
        jwt_access_secret: env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set"),
        jwt_refresh_secret: env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set"),
        discord_client_secret: env::var("OAUTH_DISCORD_CLIENT_SECRET")
            .expect("DISCORD_CLIENT_SECRET must be set"),
        discord_client_id: env::var("OAUTH_DISCORD_CLIENT_ID")
            .expect("DISCORD_CLIENT_ID must be set"),
        github_client_secret: env::var("OAUTH_GITHUB_CLIENT_SECRET")
            .expect("GITHUB_CLIENT_SECRET must be set"),
        github_client_id: env::var("OAUTH_GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set"),
        google_client_secret: env::var("OAUTH_GOOGLE_CLIENT_SECRET")
            .expect("GOOGLE_CLIENT_SECRET must be set"),
        google_client_id: env::var("OAUTH_GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
        resend_api_key: env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set"),
        server_mode: env::var("SERVER_MODE")
            .expect("SERVER_MODE must be set")
            .parse()
            .unwrap(),
        agent_config_uk: env::var("AGENT_CONFIG_UK").expect("AGENT_CONFIG_UK must be set"),
    }
});

pub fn load_env() {
    if cfg!(test) {
        dotenvy::from_filename(".env.test").ok();
    } else {
        dotenvy::from_filename(".env").ok();
    }
}
