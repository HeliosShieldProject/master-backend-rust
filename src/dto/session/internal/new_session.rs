use crate::data::{enums::SessionStatus, schema};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable, Clone)]
#[diesel(table_name = schema::Session)]
pub struct NewSession {
    pub status: SessionStatus,
    pub device_id: Uuid,
    pub config_id: Uuid,
}
