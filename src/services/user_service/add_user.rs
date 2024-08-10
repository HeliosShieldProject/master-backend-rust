use diesel::prelude::*;
use tracing::{error, info};

use super::get_by_email;
use crate::{
    data::{models::User, schema},
    dto::auth::internal::FullUser,
    enums::errors::internal::{Auth, Error, Result},
};

pub async fn add_user(pool: &deadpool_diesel::postgres::Pool, email: &str) -> Result<FullUser> {
    let conn = pool.get().await?;
    let email = email.to_owned();

    if get_by_email(pool, &email).await.is_ok() {
        error!("User already exists: {}", email);
        return Err(Error::Auth(Auth::UserAlreadyExists));
    }

    let user: User = conn
        .interact(move |conn| {
            diesel::insert_into(schema::user::table)
                .values(schema::user::email.eq(email))
                .get_result(conn)
        })
        .await??;

    info!("Added user: {}", user.id);

    Ok(FullUser {
        user,
        oauth: None,
        classic_auth: None,
    })
}
