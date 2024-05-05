use crate::{
    data::{
        enums::SessionStatus,
        models::{Config, Device, Server, Session},
        schema,
    },
    dto::session::SessionBy,
    enums::errors::internal::{to_internal, InternalError, SessionError},
    logger::{enums::Services::SessionService, ResultExt},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use uuid::Uuid;

pub struct ActiveSessionAndDevice {
    pub device_id: Uuid,
}

impl SessionBy for ActiveSessionAndDevice {
    async fn get_session<'a>(
        &self,
        pool: &'a deadpool_diesel::postgres::Pool,
    ) -> Result<(Session, Device, Config, Server), InternalError> {
        let conn = pool
            .get()
            .await
            .map_err(to_internal)
            .log_error(SessionService)
            .await?;
        let device_id = self.device_id.clone();
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
            .await
            .map_err(to_internal)
            .log(
                format!("Got active session by device_id: {}", device_id),
                SessionService,
            )
            .await?
            .map_err(|_| InternalError::Internal)?;
        if result.len() != 1 {
            return Err(InternalError::SessionError(SessionError::SessionNotFound));
        }
        Ok(result.first().unwrap().clone())
    }
}
