use once_cell::sync::Lazy;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    /// The port the master backend listens on.
    pub master_backend_url: String,
    /// The URL of the database.
    pub database_url: String,
    /// The URL of the metrics server.
    pub master_metrics_url: String,
    /// The secret used to sign access tokens.
    pub jwt_access_secret: String,
    /// The secret used to sign refresh tokens.
    pub jwt_refresh_secret: String,
    pub discord_client_secret: String,
    pub discord_client_id: String,
    pub github_client_secret: String,
    pub github_client_id: String,
    pub google_client_secret: String,
    pub google_client_id: String,
}

pub static ENV: Lazy<Config> = Lazy::new(|| {
    load_env();
    Config {
        master_backend_url: env::var("MASTER_BACKEND_URL").expect("MASTER_BACKEND_URL must be set"),
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        master_metrics_url: env::var("MASTER_METRICS_URL").expect("MASTER_METRICS_URL must be set"),
        jwt_access_secret: env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set"),
        jwt_refresh_secret: env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set"),
        discord_client_secret: env::var("DISCORD_CLIENT_SECRET")
            .expect("DISCORD_CLIENT_SECRET must be set"),
        discord_client_id: env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set"),
        github_client_secret: env::var("GITHUB_CLIENT_SECRET")
            .expect("GITHUB_CLIENT_SECRET must be set"),
        github_client_id: env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set"),
        google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
            .expect("GOOGLE_CLIENT_SECRET must be set"),
        google_client_id: env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
    }
});

pub fn load_env() {
    if cfg!(test) {
        dotenvy::from_filename(".env.test").ok();
    } else {
        dotenvy::from_filename(".env").ok();
    }
}
