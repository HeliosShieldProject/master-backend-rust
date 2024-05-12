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
    enums::errors::internal::{to_internal, InternalError, SessionError},
    services::config_service::get_config_by_country,
};
use chrono::Local;
use diesel::prelude::*;
use tracing::{error, info};
use uuid::Uuid;

pub async fn create_session(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
    country: &Country,
) -> Result<response::Session, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let (device_id, country) = (device_id.clone(), country.clone());

    if let Ok(current_session) = get_session(
        &pool,
        ActiveSessionAndDeviceAndCountry { device_id, country },
    )
    .await
    {
        let (session, _device, config, server) = current_session;
        let response = response::Session::new(session.clone(), server, config);
        info!("Found active session with the same country: {}", session.id);
        return Ok(response);
    }

    if let Ok((session, _, _, _)) = get_session(&pool, ActiveSessionAndDevice { device_id }).await {
        let _ = close_session_by_id(&pool, &session.id).await?;
    }

    let (config, server) = get_config_by_country(&pool, &country).await?;
    let new_session = NewSession {
        status: SessionStatus::Active,
        device_id: device_id.clone(),
        config_id: config.id.clone(),
    };

    conn.interact(move |conn| {
        let session = diesel::insert_into(schema::session::table)
            .values(&new_session)
            .get_result::<Session>(conn);
        let _ = diesel::update(schema::config::table)
            .filter(schema::config::id.eq(config.id))
            .set(schema::config::status.eq(ConfigStatus::InUse))
            .execute(conn);

        session
    })
    .await
    .map_err(|e| {
        error!("Error creating session: {:?}", e);
        e
    })
    .map_err(to_internal)?
    .map_err(|_| InternalError::Internal)
    .map(|session: Session| {
        info!("Created session: {}", &session.id);
        response::Session::new(session, server, config)
    })
}

pub async fn close_session_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    session_id: &Uuid,
) -> Result<Uuid, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let session_id = session_id.clone();

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
    .await
    .map_err(|e| {
        error!("Error closing session: {:?}", e);
        e
    })
    .map_err(to_internal)
    .map(|_| {
        info!("Closed session: {}", session_id);
        session_id
    })
}

pub async fn close_session(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
) -> Result<Uuid, InternalError> {
    let device_id = device_id.clone();
    let (session, _, _, _) = get_session(&pool, ActiveSessionAndDevice { device_id }).await?;
    let session_id = close_session_by_id(&pool, &session.id).await?;
    Ok(session_id)
}
