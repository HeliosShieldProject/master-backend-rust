use diesel::prelude::*;
use tracing::info;
use uuid::Uuid;

use crate::{
    agent_api,
    data::{models::Session, schema},
    enums::errors::internal::{self, Error, Result},
};

pub async fn close_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    agent_state: &agent_api::AgentState,
    session_id: &Uuid,
) -> Result<Uuid> {
    let conn = pool.get().await?;
    let session_id = *session_id;

    let session = conn
        .interact(move |conn| {
            schema::session::table
                .filter(schema::session::id.eq(session_id))
                .first::<Session>(conn)
        })
        .await?
        .map_err(|_| Error::Session(internal::Session::NotFound))?;

    agent_api::requests::delete_client(
        agent_state,
        &session.country,
        &session.protocol,
        &session.device_id,
    )
    .await?;

    info!("Closed session: {}", session_id);

    Ok(session_id)
}
