use crate::data::{enums::ConfigStatus, schema};
use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = schema::config)]
#[diesel(belongs_to(crate::dto::server::Server,))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Config {
    pub id: Uuid,
    pub private_key: String,
    pub user_ip: String,
    pub server_id: Uuid,
    pub status: ConfigStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
