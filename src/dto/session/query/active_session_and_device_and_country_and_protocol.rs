use diesel::{prelude::*, QueryDsl};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    data::{
        enums::{Country, Protocol, SessionStatus},
        models::{Device, Session},
        schema,
    },
    dto::session::SessionBy,
    enums::errors::internal::{self, Error},
};

pub struct ActiveSessionAndDeviceAndCountryAndProtocol {
    pub device_id: Uuid,
    pub country: Country,
    pub protocol: Protocol,
}

impl SessionBy for ActiveSessionAndDeviceAndCountryAndProtocol {
    async fn get_session<'a>(
        &self,
        pool: &'a deadpool_diesel::postgres::Pool,
    ) -> Result<(Session, Device), Error> {
        let conn = pool.get().await?;

        let (device_id, country, protocol) = (self.device_id, self.country, self.protocol);
        let result: Vec<(Session, Device)> = conn
            .interact(move |conn| {
                schema::session::table
                    .inner_join(schema::device::table)
                    .filter(schema::session::device_id.eq(device_id))
                    .filter(schema::session::status.eq(SessionStatus::Active))
                    .filter(schema::session::country.eq(country))
                    .filter(schema::session::protocol.eq(protocol))
                    .select((Session::as_select(), Device::as_select()))
                    .load::<(Session, Device)>(conn)
            })
            .await??;

        if result.len() != 1 {
            error!(
                "Session not found for device_id: {} and country: {}",
                device_id, country
            );

            return Err(Error::Session(internal::Session::NotFound));
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
