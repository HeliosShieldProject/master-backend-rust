use reqwest::Client;

use crate::{
    dto::auth::internal::OAuthUser,
    enums::errors::internal::{Auth, Error, Result},
};

pub async fn get_user_info_google(access_token: &str) -> Result<OAuthUser> {
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
