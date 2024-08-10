use chrono::Local;
use diesel::prelude::*;
use tracing::info;
use uuid::Uuid;

use crate::{
    data::{
        enums::{ConfigStatus, Country, SessionStatus},
        models::{Config, Server, Session},
        schema,
    },
    dto::session::{
        interface::get_session,
        internal::{NewSession, SessionHistory},
        query::{ActiveSessionAndDevice, ActiveSessionAndDeviceAndCountry},
        request::Params,
        response,
    },
    enums::errors::internal::{InternalError, Result, SessionError},
    services::config_service::get_config_by_country,
};

pub async fn create_session(
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
) -> Result<Uuid> {
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

pub async fn get_history(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
    params: &Params,
) -> Result<Vec<SessionHistory>> {
    let conn = pool.get().await?;

    let limit = params.limit.unwrap_or(10);
    let offset = params.offset.unwrap_or(0);
    let devices = params.devices.clone();
    let countries = params.countries.clone();
    let user_id = *user_id;

    let data: Vec<(Session, Config, Server)> = conn
        .interact(move |conn| {
            schema::session::table
                .inner_join(schema::config::table.inner_join(schema::server::table))
                .filter(schema::session::status.eq(SessionStatus::Closed))
                .filter(
                    schema::server::country.eq_any(
                        countries.unwrap_or(
                            schema::server::table
                                .select(schema::server::country)
                                .load::<Country>(conn)?,
                        ),
                    ),
                )
                .filter(
                    schema::session::device_id.eq_any(
                        devices.unwrap_or(
                            schema::device::table
                                .select(schema::device::id)
                                .filter(schema::device::user_id.eq(user_id))
                                .load::<Uuid>(conn)?,
                        ),
                    ),
                )
                .limit(limit)
                .offset(offset)
                .select((
                    Session::as_select(),
                    Config::as_select(),
                    Server::as_select(),
                ))
                .load::<(Session, Config, Server)>(conn)
        })
        .await??;

    let history = data.iter().map(|(session, _, server)| SessionHistory {
        id: session.id,
        device_id: session.device_id,
        opened_at: session.opened_at,
        closed_at: session.closed_at.unwrap(),
        duration: (session.closed_at.unwrap() - session.opened_at).num_seconds(),
        country: server.country,
    });

    Ok(history.collect())
}
