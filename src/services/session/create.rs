use diesel::prelude::*;
use tracing::info;
use uuid::Uuid;

use super::close_by_id;
use crate::{
    data::{
        enums::{ConfigStatus, Country, SessionStatus},
        models::Session,
        schema,
    },
    dto::session::{
        interface::get_session,
        internal::NewSession,
        query::{ActiveSessionAndDevice, ActiveSessionAndDeviceAndCountry},
        response,
    },
    enums::errors::internal::Result,
    services::config::get_by_country,
};

pub async fn create(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
    country: &Country,
) -> Result<response::Session> {
    let conn = pool.get().await?;
    let (device_id, country) = (*device_id, *country);

    if let Ok(current_session) = get_session(
        pool,
        ActiveSessionAndDeviceAndCountry { device_id, country },
    )
    .await
    {
        let (session, _device, config, server) = current_session;
        let response = response::Session::new(session.clone(), server, config);
        info!("Found active session with the same country: {}", session.id);
        return Ok(response);
    }

    if let Ok((session, _, _, _)) = get_session(pool, ActiveSessionAndDevice { device_id }).await {
        let _ = close_by_id(pool, &session.id).await?;
    }

    let (config, server) = get_by_country(pool, &country).await?;
    let new_session = NewSession {
        status: SessionStatus::Active,
        device_id,
        config_id: config.id,
    };

    let session: Session = conn
        .interact(move |conn| {
            let session = diesel::insert_into(schema::session::table)
                .values(&new_session)
                .get_result::<Session>(conn);
            let _ = diesel::update(schema::config::table)
                .filter(schema::config::id.eq(config.id))
                .set(schema::config::status.eq(ConfigStatus::InUse))
                .execute(conn);

            session
        })
        .await??;

    info!("Created session: {}", session.id);

    Ok(response::Session::new(session, server, config))
}
