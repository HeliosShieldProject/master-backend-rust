use std::time::SystemTime;

use crate::data::{enums::Country, schema};
use diesel::{Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::Server)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Server {
    pub id: Uuid,
    pub public_key: String,
    pub backend_uri: String,
    pub wireguard_uri: String,
    pub country: Country,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
