use axum::extract::FromRef;
use deadpool_diesel::postgres::{Manager, Pool};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use crate::config::ENV;

#[derive(Clone)]
pub struct OAuthProviders {
    pub discord: BasicClient,
    pub github: BasicClient,
    pub google: BasicClient,
}

impl OAuthProviders {
    pub fn default() -> Self {
        Self {
            discord: Self::client(
                &ENV.discord_client_id,
                &ENV.discord_client_secret,
                "https://discord.com/api/oauth2/authorize",
                "https://discord.com/api/oauth2/token",
            ),
            github: Self::client(
                &ENV.github_client_id,
                &ENV.github_client_secret,
                "https://github.com/login/oauth/authorize",
                "https://github.com/login/oauth/access_token",
            ),
            google: Self::client(
                &ENV.google_client_id,
                &ENV.google_client_secret,
                "https://accounts.google.com/o/oauth2/v2/auth",
                "https://www.googleapis.com/oauth2/v3/token",
            ),
        }
    }

    fn client(
        client_id: &str,
        client_secret: &str,
        auth_url: &str,
        token_url: &str,
    ) -> BasicClient {
        BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new(auth_url.to_string()).unwrap(),
            Some(TokenUrl::new(token_url.to_string()).unwrap()),
        )
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub oauth_providers: OAuthProviders,
}

impl AppState {
    pub fn default() -> Self {
        let manager = Manager::new(&ENV.database_url, deadpool_diesel::Runtime::Tokio1);
        let pool = Pool::builder(manager).build().unwrap();
        Self {
            pool,
            oauth_providers: OAuthProviders::default(),
        }
    }
}

impl FromRef<AppState> for Pool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for OAuthProviders {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_providers.clone()
    }
}
