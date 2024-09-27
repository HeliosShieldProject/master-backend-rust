use tracing::info;
use uuid::Uuid;

use super::close_by_id;
use crate::{
    agent_api,
    data::{
        enums::{Country, Protocol, SessionStatus},
        models::Session,
        schema,
    },
    dto::session::{
        interface::get_session,
        internal::NewSession,
        query::{ActiveSessionAndDevice, ActiveSessionAndDeviceAndCountryAndProtocol},
        response,
    },
    enums::errors::internal::Result,
};

pub async fn create(
    pool: &deadpool_diesel::postgres::Pool,
    agent_state: agent_api::AgentState,
    country: &Country,
    protocol: &Protocol,
    device_id: &Uuid,
) -> Result<response::Session> {
    let conn = pool.get().await?;
    let (device_id, country, protocol) = (*device_id, *country, *protocol);

    if let Ok(current_session) = get_session(
        pool,
        ActiveSessionAndDeviceAndCountryAndProtocol {
            device_id,
            country,
            protocol,
        },
    )
    .await
    {
        let (session, _device) = current_session;
        let response = response::Session::from(session);
        info!(
            "Found active session with the same country and protocol: {}",
            session.id
        );
        return Ok(response);
    }

    if let Ok((session, _)) = get_session(pool, ActiveSessionAndDevice { device_id }).await {
        let _ = close_by_id(pool, &session.id).await?;
    }

    let new_client =
        agent_api::requests::create_client(agent_state, country, protocol, &device_id).await?;
    let new_session: Session = conn
        .interact(move |conn| {
            diesel::insert_into(schema::session::table)
                .values(&NewSession {
                    session_status: SessionStatus::Active,
                    device_id,
                    country,
                    protocol,
                    link: new_client.link,
                })
                .get_result(conn)
        })
        .await??;

    info!("Created session: {}", new_session.id);

    Ok(response::Session::from(new_session))
}
