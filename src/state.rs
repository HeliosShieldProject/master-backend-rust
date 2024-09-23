use axum::extract::FromRef;
use deadpool_diesel::postgres::{Manager, Pool};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use resend_rs::Resend;

use crate::{config::ENV, data::enums::OAuthProvider};

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
                "http://localhost:3000/auth/discord/authorized",
            ),
            github: Self::client(
                &ENV.github_client_id,
                &ENV.github_client_secret,
                "https://github.com/login/oauth/authorize",
                "https://github.com/login/oauth/access_token",
                "http://localhost:3000/auth/github/authorized",
            ),
            google: Self::client(
                &ENV.google_client_id,
                &ENV.google_client_secret,
                "https://accounts.google.com/o/oauth2/v2/auth",
                "https://www.googleapis.com/oauth2/v4/token",
                "http://localhost:3000/auth/google/authorized",
            ),
        }
    }

    fn client(
        client_id: &str,
        client_secret: &str,
        auth_url: &str,
        token_url: &str,
        redirect_url: &str,
    ) -> BasicClient {
        BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new(auth_url.to_string()).unwrap(),
            Some(TokenUrl::new(token_url.to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url.to_string()).unwrap())
    }

    pub fn get(&self, provider: OAuthProvider) -> &BasicClient {
        match provider {
            OAuthProvider::Discord => &self.discord,
            OAuthProvider::Github => &self.github,
            OAuthProvider::Google => &self.google,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub oauth_providers: OAuthProviders,
    pub resend: Resend,
    pub reqwest_client: reqwest::Client,
}

impl AppState {
    pub fn default() -> Self {
        let manager = Manager::new(&ENV.database_url, deadpool_diesel::Runtime::Tokio1);
        let pool = Pool::builder(manager).build().unwrap();
        let resend = Resend::new(&ENV.resend_api_key);
        let reqwest_client = reqwest::Client::new();
        Self {
            pool,
            resend,
            oauth_providers: OAuthProviders::default(),
            reqwest_client,
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

impl FromRef<AppState> for Resend {
    fn from_ref(state: &AppState) -> Self {
        state.resend.clone()
    }
}

impl FromRef<AppState> for reqwest::Client {
    fn from_ref(state: &AppState) -> Self {
        state.reqwest_client.clone()
    }
}
