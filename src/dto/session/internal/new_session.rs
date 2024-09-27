use diesel::prelude::*;
use uuid::Uuid;

use crate::data::{
    enums::{Country, Protocol, SessionStatus},
    schema,
};

#[derive(Insertable, Clone)]
#[diesel(table_name = schema::session)]
pub struct NewSession {
    pub session_status: SessionStatus,
    pub device_id: Uuid,
    pub country: Country,
    pub protocol: Protocol,
    pub link: String,
}
