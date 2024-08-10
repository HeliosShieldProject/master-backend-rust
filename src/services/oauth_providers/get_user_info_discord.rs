use reqwest::Client;

use crate::{
    dto::auth::internal::OAuthUser,
    enums::errors::internal::{Auth, Error, Result},
};

pub async fn get_user_info_discord(access_token: &str) -> Result<OAuthUser> {
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
