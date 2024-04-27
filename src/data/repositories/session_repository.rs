use super::{
    config_repository::get_config_by_country, config_repository::Config, device_repository::Device,
    server_repository::Server,
};
use crate::data::enums::ConfigStatus;
use crate::handlers::session::create_session::Response;
use crate::{
    data::{
        enums::{Country, SessionStatus},
        schema,
    },
    enums::errors::internal::{to_internal, InternalError, SessionError},
};
use diesel::prelude::*;
use diesel::{QueryDsl, Queryable, Selectable};
use chrono::{Local, NaiveDateTime};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::Session)]
#[diesel(belongs_to(super::device_repository::Device))]
#[diesel(belongs_to(super::config_repository::Config))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub status: SessionStatus,
    pub opened_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
    pub device_id: Uuid,
    pub config_id: Uuid,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = schema::Session)]
pub struct NewSession {
    pub status: SessionStatus,
    pub device_id: Uuid,
    pub config_id: Uuid,
}

struct ActiveSessionAndDeviceAndCountry {
    device_id: Uuid,
    country: Country,
}

struct ActiveSessionAndDevice {
    device_id: Uuid,
}

trait SessionRepository {
    async fn get_session<'a>(
        &self,
        pool: &'a deadpool_diesel::postgres::Pool,
    ) -> Result<(Session, Device, Config, Server), InternalError>;
}

impl SessionRepository for ActiveSessionAndDeviceAndCountry {
    async fn get_session<'a>(
        &self,
        pool: &'a deadpool_diesel::postgres::Pool,
    ) -> Result<(Session, Device, Config, Server), InternalError> {
        let conn = pool.get().await.map_err(to_internal)?;
        let (device_id, country) = (self.device_id.clone(), self.country.clone());
        let result: Vec<(Session, Device, Config, Server)> = conn
            .interact(move |conn| {
                schema::Session::table
                    .inner_join(schema::Device::table)
                    .inner_join(schema::Config::table.inner_join(schema::Server::table))
                    .filter(schema::Session::device_id.eq(device_id))
                    .filter(schema::Session::status.eq(SessionStatus::Active))
                    .filter(schema::Server::country.eq(country))
                    .select((
                        Session::as_select(),
                        Device::as_select(),
                        Config::as_select(),
                        Server::as_select(),
                    ))
                    .load::<(Session, Device, Config, Server)>(conn)
            })
            .await
            .map_err(to_internal)?
            .map_err(|_| InternalError::Internal)?;
        if result.len() != 1 {
            return Err(InternalError::SessionError(SessionError::SessionNotFound));
        }
        Ok(result.first().unwrap().clone())
    }
}

impl SessionRepository for ActiveSessionAndDevice {
    async fn get_session<'a>(
        &self,
        pool: &'a deadpool_diesel::postgres::Pool,
    ) -> Result<(Session, Device, Config, Server), InternalError> {
        let conn = pool.get().await.map_err(to_internal)?;
        let device_id = self.device_id.clone();
        let result: Vec<(Session, Device, Config, Server)> = conn
            .interact(move |conn| {
                schema::Session::table
                    .inner_join(schema::Device::table)
                    .inner_join(schema::Config::table.inner_join(schema::Server::table))
                    .filter(schema::Session::device_id.eq(device_id))
                    .filter(schema::Session::status.eq(SessionStatus::Active))
                    .select((
                        Session::as_select(),
                        Device::as_select(),
                        Config::as_select(),
                        Server::as_select(),
                    ))
                    .load::<(Session, Device, Config, Server)>(conn)
            })
            .await
            .map_err(to_internal)?
            .map_err(|_| InternalError::Internal)?;
        if result.len() != 1 {
            return Err(InternalError::SessionError(SessionError::SessionNotFound));
        }
        Ok(result.first().unwrap().clone())
    }
}

async fn get_session<T: SessionRepository>(
    pool: &deadpool_diesel::postgres::Pool,
    by: T,
) -> Result<(Session, Device, Config, Server), InternalError> {
    let result = by.get_session(pool).await?;
    Ok(result)
}

pub async fn create_session(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
    country: &Country,
) -> Result<Response, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let (device_id, country) = (device_id.clone(), country.clone());

    if let Ok(current_session) = get_session(
        &pool,
        ActiveSessionAndDeviceAndCountry { device_id, country },
    )
    .await
    {
        let (session, _device, config, server) = current_session;
        let response = Response {
            session_id: session.id,
            server_public_key: server.public_key,
            wireguard_uri: server.wireguard_uri,
            user_ip: config.user_ip,
            user_private_key: config.private_key,
        };
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
            let session = diesel::insert_into(schema::Session::table)
                .values(&new_session)
                .get_result::<Session>(conn);
            let _ = diesel::update(schema::Config::table)
                .filter(schema::Config::id.eq(config.id))
                .set(schema::Config::status.eq(ConfigStatus::InUse))
                .execute(conn);

            session
        })
        .await
        .map_err(to_internal)?
        .map_err(|_| InternalError::Internal)?;

    Ok(Response {
        session_id: session.id,
        server_public_key: server.public_key,
        wireguard_uri: server.wireguard_uri,
        user_ip: config.user_ip,
        user_private_key: config.private_key,
    })
}

pub async fn close_session_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    session_id: &Uuid,
) -> Result<(), InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let session_id = session_id.clone();
    let _ = conn
        .interact(move |conn| {
            let session = match diesel::update(schema::Session::table)
                .filter(schema::Session::id.eq(session_id))
                .set((
                    schema::Session::status.eq(SessionStatus::Closed),
                    schema::Session::closed_at.eq(Local::now().naive_local()),
                ))
                .get_result::<Session>(conn)
                .map_err(|_| InternalError::SessionError(SessionError::SessionNotFound))
            {
                Ok(session) => session,
                Err(_) => return Err(InternalError::SessionError(SessionError::SessionNotFound)),
            };
            let _ = diesel::update(schema::Config::table)
                .filter(schema::Config::id.eq(session.config_id))
                .set(schema::Config::status.eq(ConfigStatus::NotInUse))
                .execute(conn);
            Ok(())
        })
        .await
        .map_err(to_internal)?;
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
