use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};

use super::{get_user_info_discord, get_user_info_github, get_user_info_google};
use crate::{
    data::enums::OAuthProvider,
    dto::auth::internal::{OAuthCode, OAuthUser},
    enums::errors::internal::{Error, Reqwest, Result},
    state::AppState,
};

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
