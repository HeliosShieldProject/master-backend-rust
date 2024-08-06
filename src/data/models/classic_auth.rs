use crate::data::schema;
use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::classic_auth)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassicAuth {
    pub id: Uuid,
    pub user_id: Uuid,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
