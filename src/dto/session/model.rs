use crate::{
    data::{enums::SessionStatus, schema},
    dto::{config::Config, device::Device, server::Server},
    enums::errors::internal::InternalError,
};
use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::Session)]
#[diesel(belongs_to(crate::dto::device::Device))]
#[diesel(belongs_to(crate::dto::config::Config))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub status: SessionStatus,
    pub opened_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
    pub device_id: Uuid,
    pub config_id: Uuid,
}

pub trait SessionBy {
    async fn get_session<'a>(
        &self,
        pool: &'a deadpool_diesel::postgres::Pool,
    ) -> Result<(Session, Device, Config, Server), InternalError>;
}

pub async fn get_session<T: SessionBy>(
    pool: &deadpool_diesel::postgres::Pool,
    by: T,
) -> Result<(Session, Device, Config, Server), InternalError> {
    by.get_session(pool).await
}
