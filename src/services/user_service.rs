use crate::{
    data::{enums::OS, models::User, schema},
    dto::{
        auth::{
            internal::{NewUser, OAuthCode},
            response::Tokens,
        },
        device::internal::{DeviceInfo, NewDevice},
    },
    enums::errors::{
        internal::{AuthError, InternalError},
        LogError,
    },
    services::device_service,
    state::AppState,
    utils::{hash, token::generate_tokens},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use tracing::{error, info};
use uuid::Uuid;

pub async fn get_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    id: &Uuid,
) -> Result<User, InternalError> {
    let conn = pool.get().await?;
    let id = *id;

    let user = conn
        .interact(move |conn| {
            schema::user::table
                .find(id)
                .select(User::as_select())
                .first(conn)
        })
        .await
        .log_error(&format!("User not found by id: {}", id))?
        .map_err(|_| InternalError::AuthError(AuthError::UserNotFound))?;

    info!("Got user by id: {}", user.id);

    Ok(user)
}

pub async fn get_by_email(
    pool: &deadpool_diesel::postgres::Pool,
    email: &str,
) -> Result<User, InternalError> {
    let conn = pool.get().await?;
    let email_ = email.to_string();

    let user = conn
        .interact(move |conn| {
            schema::user::table
                .filter(schema::user::email.eq(email_))
                .select(User::as_select())
                .first(conn)
        })
        .await
        .log_error(&format!("User not found by email: {}", email))?
        .map_err(|_| InternalError::AuthError(AuthError::UserNotFound))?;

    info!("Got user by email: {}", user.id);

    Ok(user)
}

pub async fn add_user(
    pool: &deadpool_diesel::postgres::Pool,
    new_user: &NewUser,
) -> Result<User, InternalError> {
    let conn = pool.get().await?;
    let new_user = new_user.clone();

    let user: User = conn
        .interact(move |conn| {
            diesel::insert_into(schema::user::table)
                .values(new_user)
                .get_result(conn)
        })
        .await?
        .log_error("User not added")?;

    info!("Added user: {}", user.id);

    Ok(user)
}

pub async fn change_password(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
    new_password: &str,
) -> Result<User, InternalError> {
    let conn = pool.get().await?;
    let user = get_by_id(pool, user_id).await?;

    if hash::verify_password(new_password, &user.password)
        .await
        .is_ok()
    {
        error!("Password is the same");
        return Err(InternalError::AuthError(AuthError::PasswordIsSame));
    }
    let hashed_password = hash::hash_password(new_password).await?;
    let id = *user_id;

    let user: User = conn
        .interact(move |conn| {
            diesel::update(schema::user::table.find(id))
                .set(schema::user::password.eq(hashed_password))
                .get_result(conn)
        })
        .await?
        .log_error(&format!("Password not changed for user: {}", id))?;

    info!("Changed password for user: {}", user.id);

    Ok(user)
}

pub async fn sign_in(
    pool: &deadpool_diesel::postgres::Pool,
    user: &NewUser,
    device: &DeviceInfo,
) -> Result<Tokens, InternalError> {
    let user_db = get_by_email(pool, &user.email).await?;

    hash::verify_password(&user.password, &user_db.password)
        .await
        .log_error("Password is incorrect")
        .map_err(|_| InternalError::AuthError(AuthError::WrongPassword))?;

    let device = NewDevice {
        name: device.name.clone(),
        os: OS::from_str(&device.os),
        user_id: user_db.id,
    };
    let device = device_service::add_device(pool, &device).await?;

    let tokens = generate_tokens(&user_db.id.to_string(), &device.id.to_string()).await?;

    info!("User signed in: {}", user_db.id);

    Ok(tokens)
}

pub async fn sign_up(
    pool: &deadpool_diesel::postgres::Pool,
    user: &NewUser,
    device: &DeviceInfo,
) -> Result<Tokens, InternalError> {
    if get_by_email(pool, &user.email).await.is_ok() {
        error!("User already exists: {}", user.email);
        return Err(InternalError::AuthError(AuthError::UserAlreadyExists));
    }

    let hashed_password = hash::hash_password(&user.password).await?;

    let new_user = NewUser {
        email: user.email.clone(),
        password: hashed_password.clone(),
    };
    let user = add_user(pool, &new_user).await?;

    let device = NewDevice {
        name: device.name.clone(),
        os: OS::from_str(&device.os),
        user_id: user.id,
    };
    let device = device_service::add_device(pool, &device).await?;

    let tokens = generate_tokens(&user.id.to_string(), &device.id.to_string()).await?;

    info!("User signed up: {}", user.id);
    Ok(tokens)
}

pub async fn authorize(
    state: &AppState,
    code: &OAuthCode,
    device: &DeviceInfo,
) -> Result<Tokens, InternalError> {
    todo!()
}
