use diesel::{prelude::*, QueryDsl};
use tracing::info;
use uuid::Uuid;

use crate::{
    data::{
        models::{ClassicAuth, OAuth, User},
        schema,
    },
    dto::auth::internal::FullUser,
    enums::errors::internal::{Auth, Error, Result},
};

pub async fn get_by_id(pool: &deadpool_diesel::postgres::Pool, id: &Uuid) -> Result<FullUser> {
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
        .map_err(|_| Error::Auth(Auth::UserNotFound))??;

    let oauth: Option<Vec<OAuth>> = conn
        .interact(move |conn| {
            schema::oauth::table
                .filter(schema::oauth::user_id.eq(id))
                .select(OAuth::as_select())
                .load::<OAuth>(conn)
                .optional()
        })
        .await??;

    let classic_auth: Option<ClassicAuth> = conn
        .interact(move |conn| {
            schema::classic_auth::table
                .filter(schema::classic_auth::user_id.eq(id))
                .select(ClassicAuth::as_select())
                .first(conn)
                .optional()
        })
        .await??;

    info!("Got user by id: {}", user.id);

    Ok(FullUser {
        user,
        oauth,
        classic_auth,
    })
}
