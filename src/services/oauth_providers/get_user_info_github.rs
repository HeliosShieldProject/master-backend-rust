use reqwest::Client;

use crate::{
    dto::auth::internal::OAuthUser,
    enums::errors::internal::{Auth, Error, Result},
};

pub async fn get_user_info_github(access_token: &str) -> Result<OAuthUser> {
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
