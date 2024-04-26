use std::time::SystemTime;

use crate::data::{enums::ConfigStatus, schema};
use diesel::{Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::Config)]
#[diesel(belongs_to(super::server_repository::Server))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Config {
    pub id: Uuid,
    pub private_key: String,
    pub user_ip: String,
    pub server_id: Uuid,
    pub status: ConfigStatus,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
