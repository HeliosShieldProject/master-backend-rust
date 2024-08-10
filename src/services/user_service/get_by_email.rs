use diesel::{prelude::*, QueryDsl};
use tracing::info;

use crate::{
    data::{
        models::{ClassicAuth, OAuth, User},
        schema,
    },
    dto::auth::internal::FullUser,
    enums::errors::internal::{Auth, Error, Result},
};

pub async fn get_by_email(pool: &deadpool_diesel::postgres::Pool, email: &str) -> Result<FullUser> {
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
        .map_err(|_| Error::Auth(Auth::UserNotFound))??;

    let oauth: Option<Vec<OAuth>> = conn
        .interact(move |conn| {
            schema::oauth::table
                .filter(schema::oauth::user_id.eq(user.id))
                .select(OAuth::as_select())
                .load::<OAuth>(conn)
                .optional()
        })
        .await??;

    let classic_auth: Option<ClassicAuth> = conn
        .interact(move |conn| {
            schema::classic_auth::table
                .filter(schema::classic_auth::user_id.eq(user.id))
                .select(ClassicAuth::as_select())
                .first(conn)
                .optional()
        })
        .await??;

    info!("Got user by email: {}", user.id);

    Ok(FullUser {
        user,
        oauth,
        classic_auth,
    })
}
