use diesel::{prelude::*, QueryDsl};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    data::{
        enums::SessionStatus,
        models::{Config, Device, Server, Session},
        schema,
    },
    dto::session::SessionBy,
    enums::errors::internal::{self, Error},
};

pub struct ActiveSessionAndDevice {
    pub device_id: Uuid,
}

impl SessionBy for ActiveSessionAndDevice {
    async fn get_session<'a>(
        &self,
        pool: &'a deadpool_diesel::postgres::Pool,
    ) -> Result<(Session, Device, Config, Server), Error> {
        let conn = pool.get().await?;

        let device_id = self.device_id;
        let result: Vec<(Session, Device, Config, Server)> = conn
            .interact(move |conn| {
                schema::session::table
                    .inner_join(schema::device::table)
                    .inner_join(schema::config::table.inner_join(schema::server::table))
                    .filter(schema::session::device_id.eq(device_id))
                    .filter(schema::session::status.eq(SessionStatus::Active))
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
            error!("Session not found for device_id: {}", device_id);

            return Err(Error::Session(internal::Session::NotFound));
        }

        info!(
            "Found active session: {} for device_id: {}",
            result.first().unwrap().0.id,
            device_id
        );

        Ok(result.first().unwrap().clone())
    }
}
