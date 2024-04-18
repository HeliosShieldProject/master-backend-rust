use dotenvy::dotenv;
use std::env;
use tokio::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct Config {
    master_backend_port: u16,
    database_url: String,
    jwt_access_secret: String,
    jwt_refresh_secret: String,
    salt: String,
}

impl Config {
    pub fn master_backend_port(&self) -> u16 {
        self.master_backend_port
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn jwt_access_secret(&self) -> &str {
        &self.jwt_access_secret
    }

    pub fn jwt_refresh_secret(&self) -> &str {
        &self.jwt_refresh_secret
    }

    pub fn salt(&self) -> &str {
        &self.salt
    }
}

async fn init_config() -> Config {
    dotenv().ok();
    let config = Config {
        master_backend_port: env::var("MASTER_BACKEND_PORT")
            .expect("MASTER_BACKEND_PORT must be set")
            .parse()
            .expect("MASTER_BACKEND_PORT must be a number"),
        database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        jwt_access_secret: env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set"),
        jwt_refresh_secret: env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set"),
        salt: env::var("SALT").expect("SALT must be set"),
    };
    config
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}
