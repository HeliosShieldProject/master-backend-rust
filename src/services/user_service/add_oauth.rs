use diesel::prelude::*;
use tracing::{error, info};
use uuid::Uuid;

use super::get_by_id;
use crate::{
    data::{models::OAuth, schema},
    dto::auth::internal::{OAuthCode, OAuthUser},
    enums::errors::internal::{Auth, Error, Result},
    state::AppState,
};

pub async fn add_oauth(
    state: &AppState,
    oauth_user: &OAuthUser,
    oauth_code: &OAuthCode,
    user_id: &Uuid,
) -> Result<OAuth> {
    let conn = state.pool.get().await?;
    let user_id = *user_id;
    let (provider, metadata) = (oauth_code.provider, oauth_user.metadata.clone());

    let current_user = get_by_id(&state.pool, &user_id).await?;

    if current_user.user.email != oauth_user.email {
        error!(
            "OAuth email is different: {} != {}",
            current_user.user.email, oauth_user.email
        );
        return Err(Error::Auth(Auth::OAuthDifferentEmail));
    }

    if current_user.oauth.is_some()
        && current_user
            .oauth
            .unwrap()
            .iter()
            .any(|oauth| oauth.provider == provider)
    {
        let oauth: OAuth = conn
            .interact(move |conn| {
                diesel::update(schema::oauth::table)
                    .filter(schema::oauth::user_id.eq(user_id))
                    .filter(schema::oauth::provider.eq(provider))
                    .set(schema::oauth::updated_at.eq(diesel::dsl::now))
                    .get_result(conn)
            })
            .await??;
        return Ok(oauth);
    }

    let oauth: OAuth = conn
        .interact(move |conn| {
            diesel::insert_into(schema::oauth::table)
                .values((
                    schema::oauth::user_id.eq(user_id),
                    schema::oauth::provider.eq(provider),
                    schema::oauth::metadata.eq(metadata),
                ))
                .get_result(conn)
        })
        .await??;

    info!("Added OAuth for user: {}", user_id);

    Ok(oauth)
}
