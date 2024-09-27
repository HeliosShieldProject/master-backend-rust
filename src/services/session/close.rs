use uuid::Uuid;

use crate::{
    agent_api,
    dto::session::{interface::get_session, query::ActiveSessionAndDevice},
    enums::errors::internal::Result,
};

use super::close_by_id;

pub async fn close(
    pool: &deadpool_diesel::postgres::Pool,
    agent_state: agent_api::AgentState,
    device_id: &Uuid,
) -> Result<Uuid> {
    let (session, _) = get_session(
        pool,
        ActiveSessionAndDevice {
            device_id: *device_id,
        },
    )
    .await?;
    let session_id = close_by_id(pool, agent_state, &session.id).await?;
    Ok(session_id)
}
