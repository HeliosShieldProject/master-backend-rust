use crate::data::{
    enums::UserStatus,
    errors::{adapt_infra_error, InfraError},
    schema,
};
use diesel::prelude::*;
use diesel::{QueryDsl, Queryable, Selectable};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::User)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub banned_at: Option<SystemTime>,
    pub banned_till: Option<SystemTime>,
    pub status: UserStatus,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = schema::User)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

pub async fn get_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    id: &Uuid,
) -> Result<User, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let id = id.clone();
    let result = conn
        .interact(move |conn| {
            schema::User::table
                .find(id)
                .select(User::as_select())
                .first(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(result)
}

pub async fn get_by_email(
    pool: &deadpool_diesel::postgres::Pool,
    email: &str,
) -> Result<User, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let email = email.to_owned();
    let result = conn
        .interact(move |conn| {
            schema::User::table
                .filter(schema::User::email.eq(email))
                .select(User::as_select())
                .first(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(result)
}

pub async fn add_user(
    pool: &deadpool_diesel::postgres::Pool,
    new_user: &NewUser,
) -> Result<User, InfraError> {
    if get_by_email(&pool, &new_user.email).await.is_ok() {
        return Err(InfraError::NotFound);
    }
    
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let new_user = new_user.clone();
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(schema::User::table)
                .values(&new_user)
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(result)
}
