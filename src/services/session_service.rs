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
    logger::{enums::Services::SessionService, ContextLogger, ResultExt},
    services::config_service::get_config_by_country,
};
use chrono::Local;
use diesel::prelude::*;
use uuid::Uuid;

const LOG: ContextLogger = ContextLogger::new(SessionService);

pub async fn create_session(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
    country: &Country,
) -> Result<response::Session, InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(SessionService)
        .await?;
    let (device_id, country) = (device_id.clone(), country.clone());

    if let Ok(current_session) = get_session(
        &pool,
        ActiveSessionAndDeviceAndCountry { device_id, country },
    )
    .await
    {
        let (session, _device, config, server) = current_session;
        let response = response::Session::new(session, server, config);
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
    let session = conn
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
        .await
        .map_err(to_internal)
        .log_error(SessionService)
        .await?
        .map_err(|_| InternalError::Internal)?;

    LOG.info(format!("Created session: {}", &session.id)).await;
    Ok(response::Session::new(session, server, config))
}

pub async fn close_session_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    session_id: &Uuid,
) -> Result<(), InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(SessionService)
        .await?;
    let session_id = session_id.clone();
    let _ = conn
        .interact(move |conn| {
            let session = match diesel::update(schema::session::table)
                .filter(schema::session::id.eq(session_id))
                .set((
                    schema::session::status.eq(SessionStatus::Closed),
                    schema::session::closed_at.eq(Local::now().naive_local()),
                ))
                .get_result::<Session>(conn)
                .map_err(|_| InternalError::SessionError(SessionError::SessionNotFound))
            {
                Ok(session) => session,
                Err(_) => return Err(InternalError::SessionError(SessionError::SessionNotFound)),
            };
            let _ = diesel::update(schema::config::table)
                .filter(schema::config::id.eq(session.config_id))
                .set(schema::config::status.eq(ConfigStatus::NotInUse))
                .execute(conn);
            Ok(())
        })
        .await
        .map_err(to_internal)
        .log(format!("Closed session: {}", session_id), SessionService)
        .await?;
    Ok(())
}

pub async fn close_session(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
) -> Result<(), InternalError> {
    let device_id = device_id.clone();
    let (session, _, _, _) = get_session(&pool, ActiveSessionAndDevice { device_id }).await?;
    let _ = close_session_by_id(&pool, &session.id).await?;
    Ok(())
}
