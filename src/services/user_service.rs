use crate::{
    data::{models::User, schema},
    dto::auth::internal::NewUser,
    enums::errors::internal::{to_internal, AuthError, InternalError},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use uuid::Uuid;

pub async fn get_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    id: &Uuid,
) -> Result<User, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let id = id.clone();
    let result = conn
        .interact(move |conn| {
            schema::User::table
                .find(id)
                .select(User::as_select())
                .first(conn)
        })
        .await
        .map_err(to_internal)?
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
    let conn = pool.get().await.map_err(to_internal)?;
    let email = email.to_owned();
    let result = conn
        .interact(move |conn| {
            schema::User::table
                .filter(schema::User::email.eq(email))
                .select(User::as_select())
                .first(conn)
        })
        .await
        .map_err(to_internal)?
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
    let conn = pool.get().await.map_err(to_internal)?;
    let new_user = new_user.clone();
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(schema::User::table)
                .values(&new_user)
                .get_result(conn)
        })
        .await
        .map_err(to_internal)?
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
    id: &Uuid,
    new_password: &str,
) -> Result<User, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let id = id.clone();
    let new_password = new_password.to_owned();
    let result = conn
        .interact(move |conn| {
            diesel::update(schema::User::table.find(id))
                .set(schema::User::password.eq(new_password))
                .get_result(conn)
        })
        .await
        .map_err(to_internal)?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => InternalError::AuthError(AuthError::UserNotFound),
            _ => InternalError::Internal,
        })?;

    Ok(result)
}
