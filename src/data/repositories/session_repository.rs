use super::{config_repository::Config, device_repository::Device, server_repository::Server};
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
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::Session)]
#[diesel(belongs_to(super::device_repository::Device))]
#[diesel(belongs_to(super::config_repository::Config))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub status: SessionStatus,
    pub opened_at: SystemTime,
    pub closed_at: Option<SystemTime>,
    pub device_id: Uuid,
    pub config_id: Uuid,
}

struct ActiveSessionAndDeviceAndCountry {
    device_id: Uuid,
    country: Country,
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
        let result = conn
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
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    InternalError::SessionError(SessionError::SessionNotFound)
                }
                _ => InternalError::Internal,
            })?;
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

    Ok(Response {
        session_id: Uuid::new_v4(),
        server_public_key: "server_public_key".to_string(),
        wireguard_uri: "wireguard_uri".to_string(),
        user_ip: "user_ip".to_string(),
        user_private_key: "user_private_key".to_string(),
    })
}
