use chrono::Local;
use diesel::prelude::*;
use tracing::info;
use uuid::Uuid;

use crate::{
    data::{
        enums::{ConfigStatus, SessionStatus},
        models::Session,
        schema,
    },
    enums::errors::internal::{self, Error, Result},
};

pub async fn close_session_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    session_id: &Uuid,
) -> Result<Uuid> {
    let conn = pool.get().await?;
    let session_id = *session_id;

    conn.interact(move |conn| {
        let session = match diesel::update(schema::session::table)
            .filter(schema::session::id.eq(session_id))
            .set((
                schema::session::status.eq(SessionStatus::Closed),
                schema::session::closed_at.eq(Local::now().naive_local()),
            ))
            .get_result::<Session>(conn)
            .map_err(|_| Error::Session(internal::Session::SessionNotFound))
        {
            Ok(session) => {
                info!("Found session: {}", session.id);
                session
            }
            Err(_) => return Err(Error::Session(internal::Session::SessionNotFound)),
        };
        let _ = diesel::update(schema::config::table)
            .filter(schema::config::id.eq(session.config_id))
            .set(schema::config::status.eq(ConfigStatus::NotInUse))
            .execute(conn);
        Ok(())
    })
    .await??;

    info!("Closed session: {}", session_id);

    Ok(session_id)
}
