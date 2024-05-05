use crate::{
    data::{enums::OS, models::User, schema},
    dto::{
        auth::{internal::NewUser, response::Tokens},
        device::internal::{DeviceInfo, NewDevice},
    },
    enums::errors::internal::{to_internal, AuthError, InternalError},
    logger::{enums::Services::UserService, ContextLogger, ResultExt},
    services::device_service,
    utils::{hash, token::generate_tokens},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use uuid::Uuid;

const LOG: ContextLogger = ContextLogger::new(UserService);

pub async fn get_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    id: &Uuid,
) -> Result<User, InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(UserService)
        .await?;
    let id = id.clone();
    let result = conn
        .interact(move |conn| {
            schema::user::table
                .find(id)
                .select(User::as_select())
                .first(conn)
        })
        .await
        .map_err(to_internal)
        .log(format!("Got user by id: {}", id), UserService)
        .await?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => InternalError::AuthError(AuthError::UserNotFound),
            _ => InternalError::Internal,
        })?;

    Ok(result)
}

pub async fn get_by_email(
    pool: &deadpool_diesel::postgres::Pool,
    email: &str,
) -> Result<User, InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(UserService)
        .await?;
    let email_ = email.to_string();
    let result = conn
        .interact(move |conn| {
            schema::user::table
                .filter(schema::user::email.eq(email_))
                .select(User::as_select())
                .first(conn)
        })
        .await
        .map_err(to_internal)
        .log(format!("Got user by email: {}", email), UserService)
        .await?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => InternalError::AuthError(AuthError::UserNotFound),
            _ => InternalError::Internal,
        })?;

    Ok(result)
}

pub async fn add_user(
    pool: &deadpool_diesel::postgres::Pool,
    new_user: &NewUser,
) -> Result<User, InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(UserService)
        .await?;
    let new_user_ = new_user.clone();
    let result: User = conn
        .interact(move |conn| {
            diesel::insert_into(schema::user::table)
                .values(&new_user_)
                .get_result(conn)
        })
        .await
        .map_err(to_internal)
        .log(format!("Added user: {}", &new_user.email), UserService)
        .await?
        .map_err(|e| match e {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => InternalError::AuthError(AuthError::UserAlreadyExists),
            _ => InternalError::Internal,
        })?;

    Ok(result)
}

pub async fn change_password(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
    new_password: &str,
) -> Result<User, InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(UserService)
        .await?;
    let user = get_by_id(&pool, &user_id).await?;

    if hash::verify_password(&new_password, &user.password).await.is_ok() {
        return Err(InternalError::AuthError(AuthError::PasswordIsSame));
    }
    let hashed_password = hash::hash_password(&new_password).await?;
    let id = user_id.clone();
    let result = conn
        .interact(move |conn| {
            diesel::update(schema::user::table.find(id))
                .set(schema::user::password.eq(hashed_password))
                .get_result(conn)
        })
        .await
        .map_err(to_internal)
        .log(format!("Changed password for user: {}", id), UserService)
        .await?
        .map_err(|_| InternalError::Internal)?;

    Ok(result)
}

pub async fn sign_in(
    pool: &deadpool_diesel::postgres::Pool,
    user: &NewUser,
    device: &DeviceInfo,
) -> Result<Tokens, InternalError> {
    let user_db = get_by_email(&pool, &user.email).await?;

    hash::verify_password(&user.password, &user_db.password).await?;

    let device = NewDevice {
        name: device.name.clone(),
        os: OS::from_str(&device.os),
        user_id: user_db.id.clone(),
    };
    let device = device_service::add_device(&pool, &device).await?;

    let tokens = generate_tokens(&user_db.id.to_string(), &device.id.to_string()).await?;

    LOG.info(format!("User signed in: {}", user_db.id)).await;
    Ok(tokens)
}

pub async fn sign_up(
    pool: &deadpool_diesel::postgres::Pool,
    user: &NewUser,
    device: &DeviceInfo,
) -> Result<Tokens, InternalError> {
    if get_by_email(&pool, &user.email).await.is_ok() {
        return Err(InternalError::AuthError(AuthError::UserAlreadyExists));
    }

    let hashed_password = hash::hash_password(&user.password).await?;

    let new_user = NewUser {
        email: user.email.clone(),
        password: hashed_password.clone(),
    };
    let user = add_user(&pool, &new_user).await?;

    let device = NewDevice {
        name: device.name.clone(),
        os: OS::from_str(&device.os),
        user_id: user.id.clone(),
    };
    let device = device_service::add_device(&pool, &device).await?;

    let tokens = generate_tokens(&user.id.to_string(), &device.id.to_string()).await?;

    LOG.info(format!("User signed up: {}", user.id)).await;
    Ok(tokens)
}
