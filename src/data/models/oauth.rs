use crate::data::{enums::OAuthProvider, schema};
use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::oauth)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OAuth {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: OAuthProvider,
    pub metadata: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
