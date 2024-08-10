use crate::{
    data::{
        enums::{Country, SessionStatus},
        models::{Config, Device, Server, Session},
        schema,
    },
    dto::session::SessionBy,
    enums::errors::internal::{self, Error},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use tracing::{error, info};
use uuid::Uuid;

pub struct ActiveSessionAndDeviceAndCountry {
    pub device_id: Uuid,
    pub country: Country,
}

impl SessionBy for ActiveSessionAndDeviceAndCountry {
    async fn get_session<'a>(
        &self,
        pool: &'a deadpool_diesel::postgres::Pool,
    ) -> Result<(Session, Device, Config, Server), Error> {
        let conn = pool.get().await?;

        let (device_id, country) = (self.device_id, self.country);
        let result: Vec<(Session, Device, Config, Server)> = conn
            .interact(move |conn| {
                schema::session::table
                    .inner_join(schema::device::table)
                    .inner_join(schema::config::table.inner_join(schema::server::table))
                    .filter(schema::session::device_id.eq(device_id))
                    .filter(schema::session::status.eq(SessionStatus::Active))
                    .filter(schema::server::country.eq(country))
                    .select((
                        Session::as_select(),
                        Device::as_select(),
                        Config::as_select(),
                        Server::as_select(),
                    ))
                    .load::<(Session, Device, Config, Server)>(conn)
            })
            .await??;

        if result.len() != 1 {
            error!(
                "Session not found for device_id: {} and country: {}",
                device_id, country
            );

            return Err(Error::Session(internal::Session::SessionNotFound));
        }

        info!(
            "Found active session: {} for device_id: {} and country: {}",
            result.first().unwrap().0.id,
            device_id,
            country
        );

        Ok(result.first().unwrap().clone())
    }
}
