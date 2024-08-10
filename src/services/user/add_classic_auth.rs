use diesel::prelude::*;
use tracing::info;
use uuid::Uuid;

use crate::{
    data::{models::ClassicAuth, schema},
    enums::errors::internal::Result,
    utils::hash,
};

pub async fn add_classic_auth(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
    password: &str,
) -> Result<ClassicAuth> {
    let conn = pool.get().await?;
    let user_id = *user_id;
    let hashed_password = hash::hash_password(password).await?;

    let classic_auth: ClassicAuth = conn
        .interact(move |conn| {
            diesel::insert_into(schema::classic_auth::table)
                .values((
                    schema::classic_auth::user_id.eq(user_id),
                    schema::classic_auth::password_hash.eq(hashed_password),
                ))
                .get_result(conn)
        })
        .await??;

    info!("Added classic auth for user: {}", user_id);

    Ok(classic_auth)
}
