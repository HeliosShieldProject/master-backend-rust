use diesel::prelude::*;
use tracing::info;
use uuid::Uuid;

use crate::{
    data::{models::EmailConfirmation, schema},
    enums::errors::internal::Result,
};

pub async fn confirm(pool: &deadpool_diesel::postgres::Pool, token: &Uuid) -> Result<()> {
    let conn = pool.get().await?;
    let token = *token;

    let _ = conn
        .interact(move |conn| {
            diesel::update(schema::email_confirmation::table)
                .filter(schema::email_confirmation::id.eq(token))
                .set(schema::email_confirmation::confirmed.eq(true))
                .get_result::<EmailConfirmation>(conn)
        })
        .await??;

    info!("Email confirmed successfully");

    Ok(())
}
