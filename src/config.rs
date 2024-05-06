use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    /// The port the master backend listens on.
    pub master_backend_url: String,
    /// The URL of the database.
    pub database_url: String,
    /// The secret used to sign access tokens.
    pub jwt_access_secret: String,
    /// The secret used to sign refresh tokens.
    pub jwt_refresh_secret: String,
    /// Version of the rust environment.
    pub rust_env: String,
    /// The URL of the logger.
    pub logger_url: String,
}

pub static ENV: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    Config {
        master_backend_url: env::var("MASTER_BACKEND_URL").expect("MASTER_BACKEND_URL must be set"),
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        jwt_access_secret: env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set"),
        jwt_refresh_secret: env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set"),
        rust_env: env::var("RUST_ENV").expect("RUST_ENV must be set"),
        logger_url: env::var("LOGGER_URL").expect("LOGGER_URL must be set"),
    }
});
