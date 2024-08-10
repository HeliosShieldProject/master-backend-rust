use diesel::prelude::*;
use uuid::Uuid;

use crate::data::{enums::SessionStatus, schema};

#[derive(Insertable, Clone)]
#[diesel(table_name = schema::session)]
pub struct NewSession {
    pub status: SessionStatus,
    pub device_id: Uuid,
    pub config_id: Uuid,
}
