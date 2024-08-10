use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use uuid::Uuid;

use crate::data::{enums::Country, schema};

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::server)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Server {
    pub id: Uuid,
    pub public_key: String,
    pub wireguard_uri: String,
    pub country: Country,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
