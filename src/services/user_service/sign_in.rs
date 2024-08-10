use tracing::{error, info};

use crate::{
    dto::{
        auth::{internal::NewUser, response::Tokens},
        device::internal::{DeviceInfo, NewDevice},
    },
    enums::errors::internal::{Auth, Error, Result},
    services::device_service,
    utils::{hash, token::generate_tokens},
};

use super::get_by_email;

pub async fn sign_in(
    pool: &deadpool_diesel::postgres::Pool,
    user: &NewUser,
    device: &DeviceInfo,
) -> Result<Tokens> {
    let user_db = get_by_email(pool, &user.email).await.map_err(|_| {
        error!("User not found: {}", user.email);
        Error::Auth(Auth::UserNotFound)
    })?;

    if user_db.classic_auth.is_none() {
        error!("User has no classic auth: {}", user.email);
        return Err(Error::Auth(Auth::NoClassicAuth));
    }
    let classic_auth = user_db.classic_auth.unwrap();

    hash::verify_password(&user.password, &classic_auth.password_hash)
        .await
        .map_err(|_| Error::Auth(Auth::WrongPassword))?;

    let device = NewDevice {
        name: device.name.clone(),
        os: device.os,
        user_id: user_db.user.id,
    };
    let device = device_service::add_device(pool, &device).await?;

    let tokens = generate_tokens(&user_db.user.id.to_string(), &device.id.to_string()).await?;

    info!("User signed in: {}", user_db.user.id);

    Ok(tokens)
}
