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
    enums::errors::internal::{InternalError, SessionError},
    services::config_service::get_config_by_country,
};
use chrono::Local;
use diesel::prelude::*;
use tracing::info;
use uuid::Uuid;

pub async fn create_session(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
    country: &Country,
) -> Result<response::Session, InternalError> {
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
        let _ = close_session_by_id(pool, &session.id).await?;
    }

    let (config, server) = get_config_by_country(pool, &country).await?;
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

pub async fn close_session_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    session_id: &Uuid,
) -> Result<Uuid, InternalError> {
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
            .map_err(|_| InternalError::SessionError(SessionError::SessionNotFound))
        {
            Ok(session) => {
                info!("Found session: {}", session.id);
                session
            }
            Err(_) => return Err(InternalError::SessionError(SessionError::SessionNotFound)),
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

pub async fn close_session(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
) -> Result<Uuid, InternalError> {
    let (session, _, _, _) = get_session(
        pool,
        ActiveSessionAndDevice {
            device_id: *device_id,
        },
    )
    .await?;
    let session_id = close_session_by_id(pool, &session.id).await?;
    Ok(session_id)
}
