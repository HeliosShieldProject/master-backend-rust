use tracing::info;

use super::{add_oauth, add_user, get_by_email};
use crate::{
    dto::{
        auth::{
            internal::{FullUser, OAuthCode},
            response::Tokens,
        },
        device::internal::{DeviceInfo, NewDevice},
    },
    enums::errors::internal::Result,
    services::{device, oauth_providers},
    state::AppState,
    utils::token::generate_tokens,
};

pub async fn authorize(state: &AppState, code: &OAuthCode, device: &DeviceInfo) -> Result<Tokens> {
    let oauth_user = oauth_providers::authorize_user(state, code).await?;

    let current_user = get_by_email(&state.pool, &oauth_user.email).await;
    let user: FullUser;
    if current_user.is_ok() {
        add_oauth(
            state,
            &oauth_user,
            code,
            &current_user.clone().unwrap().user.id,
        )
        .await?;
        user = current_user.unwrap();
    } else {
        user = add_user(&state.pool, &oauth_user.email).await?;
        add_oauth(state, &oauth_user, code, &user.user.id).await?;
    }

    let device = NewDevice {
        name: device.name.clone(),
        os: device.os,
        user_id: user.user.id,
    };

    let device = device::add(&state.pool, &device).await?;

    let tokens = generate_tokens(&user.user.id.to_string(), &device.id.to_string()).await?;

    info!("User authorized: {}", user.user.id);

    Ok(tokens)
}
