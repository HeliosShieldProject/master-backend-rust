use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use uuid::Uuid;

use crate::data::{enums::SessionStatus, schema};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::session)]
#[diesel(belongs_to(crate::dto::device::Device))]
#[diesel(belongs_to(crate::dto::config::Config))]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Session {
    pub id: Uuid,
    pub status: SessionStatus,
    pub opened_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
    pub device_id: Uuid,
    pub config_id: Uuid,
}
