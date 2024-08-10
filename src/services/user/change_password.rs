use diesel::prelude::*;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    data::schema,
    enums::errors::internal::{Auth, Error, Result},
    utils::hash,
};

use super::get_by_id;

pub async fn change_password(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
    new_password: &str,
) -> Result<()> {
    let conn = pool.get().await?;
    let user = get_by_id(pool, user_id).await?;

    if user.classic_auth.is_none() {
        return Err(Error::Auth(Auth::NoClassicAuth));
    }

    if hash::verify_password(new_password, &user.classic_auth.unwrap().password_hash)
        .await
        .is_ok()
    {
        error!("Password is the same for user: {}", user_id);
        return Err(Error::Auth(Auth::PasswordIsSame));
    }

    let new_password_hash = hash::hash_password(new_password).await?;
    let user_id = *user_id;

    conn.interact(move |conn| {
        let _ = diesel::update(schema::user::table)
            .filter(schema::user::id.eq(user_id))
            .set(schema::user::updated_at.eq(diesel::dsl::now))
            .execute(conn);
        let _ = diesel::update(schema::classic_auth::table)
            .filter(schema::classic_auth::user_id.eq(user_id))
            .set(schema::classic_auth::password_hash.eq(new_password_hash))
            .execute(conn);
    })
    .await?;

    info!("Password changed successfully for user: {}", user_id);

    Ok(())
}
