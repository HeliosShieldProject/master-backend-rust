use tracing::{error, info};

use crate::{
    dto::{
        auth::{
            internal::{FullUser, NewUser},
            response::Tokens,
        },
        device::internal::{DeviceInfo, NewDevice},
    },
    enums::errors::internal::{Auth, Error, Result},
    services::device_service,
    utils::token::generate_tokens,
};

use super::{add_classic_auth, add_user, get_by_email, have_classic_auth, have_oauth};

pub async fn sign_up(
    pool: &deadpool_diesel::postgres::Pool,
    user: &NewUser,
    device: &DeviceInfo,
) -> Result<Tokens> {
    if have_classic_auth(pool, &user.email).await {
        error!("User already exists: {}", user.email);
        return Err(Error::Auth(Auth::UserAlreadyExists));
    }

    let current_user: FullUser = if have_oauth(pool, &user.email).await {
        get_by_email(pool, &user.email).await?
    } else {
        add_user(pool, &user.email).await?
    };

    add_classic_auth(pool, &current_user.user.id, &user.password).await?;

    let device = NewDevice {
        name: device.name.clone(),
        os: device.os,
        user_id: current_user.user.id,
    };

    let device = device_service::add_device(pool, &device).await?;

    let tokens = generate_tokens(&current_user.user.id.to_string(), &device.id.to_string()).await?;

    info!("User signed up user: {}", current_user.user.id);

    Ok(tokens)
}
