use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
use reqwest::Client;

use crate::{
    data::enums::OAuthProvider,
    dto::auth::internal::{OAuthCode, OAuthUser},
    enums::errors::internal::{Auth, Error, Reqwest, Result},
    state::AppState,
};

async fn get_user_info_google(access_token: &str) -> Result<OAuthUser> {
    let client = Client::new();
    let res = client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if let Some(email) = res["email"].as_str() {
        return Ok(OAuthUser {
            email: email.to_string(),
            metadata: res,
        });
    }

    Err(Error::Auth(Auth::OAuthFailed))
}

async fn get_user_info_discord(access_token: &str) -> Result<OAuthUser> {
    let client = Client::new();
    let res = client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if let Some(email) = res["email"].as_str() {
        return Ok(OAuthUser {
            email: email.to_string(),
            metadata: res,
        });
    }

    Err(Error::Auth(Auth::OAuthFailed))
}

async fn get_user_info_github(access_token: &str) -> Result<OAuthUser> {
    let client = Client::new();
    let res = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if let Some(email) = res["email"].as_str() {
        return Ok(OAuthUser {
            email: email.to_string(),
            metadata: res,
        });
    }

    let emails = client
        .get("https://api.github.com/user/emails")
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<Vec<serde_json::Value>>()
        .await?;

    if let Some(email) = emails.iter().find(|e| e["primary"].as_bool().unwrap()) {
        return Ok(OAuthUser {
            email: email["email"].as_str().unwrap().to_string(),
            metadata: res,
        });
    }

    Err(Error::Auth(Auth::OAuthFailed))
}

pub async fn authorize_user(state: &AppState, oauth_code: &OAuthCode) -> Result<OAuthUser> {
    let token = state
        .oauth_providers
        .get(oauth_code.provider)
        .exchange_code(AuthorizationCode::new(oauth_code.code.to_string()))
        .request_async(async_http_client)
        .await
        .map_err(|_| Error::Reqwest(Reqwest::AccessToken))?;

    match oauth_code.provider {
        OAuthProvider::Google => get_user_info_google(token.access_token().secret()).await,
        OAuthProvider::Discord => get_user_info_discord(token.access_token().secret()).await,
        OAuthProvider::Github => get_user_info_github(token.access_token().secret()).await,
    }
}
