use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use uuid::Uuid;

use crate::data::{
    enums::{Country, Protocol, SessionStatus},
    schema,
};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub device_id: Uuid,
    pub protocol: Protocol,
    pub country: Country,
    pub link: String,
    pub status: SessionStatus,
    pub up: Option<i32>,
    pub down: Option<i32>,
    pub opened_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
}
