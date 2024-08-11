use tracing::{error, info};

use super::{add_classic_auth, add_user, get_by_email, have_classic_auth, have_oauth};
use crate::{
    dto::{
        auth::{
            internal::{FullUser, NewUser},
            response::Tokens,
        },
        device::internal::{DeviceInfo, NewDevice},
    },
    enums::errors::internal::{Auth, Error, Result},
    services::{device, email},
    state::AppState,
    utils::token::generate_tokens,
};

pub async fn sign_up(state: AppState, user: &NewUser, device: &DeviceInfo) -> Result<Tokens> {
    if have_classic_auth(&state.pool, &user.email).await {
        error!("User already exists: {}", user.email);
        return Err(Error::Auth(Auth::UserAlreadyExists));
    }

    let current_user: FullUser = if have_oauth(&state.pool, &user.email).await {
        get_by_email(&state.pool, &user.email).await?
    } else {
        add_user(&state.pool, &user.email).await?
    };

    add_classic_auth(&state.pool, &current_user.user.id, &user.password).await?;

    let device = NewDevice {
        name: device.name.clone(),
        os: device.os,
        user_id: current_user.user.id,
    };

    let device = device::add(&state.pool, &device).await?;

    let tokens = generate_tokens(&current_user.user.id.to_string(), &device.id.to_string()).await?;

    info!("User signed up user: {}", current_user.user.id);

    email::send_confirmation(state, current_user.user).await?;

    Ok(tokens)
}
