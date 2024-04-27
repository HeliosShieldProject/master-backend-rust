use crate::data::{enums::UserStatus, schema};
use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub banned_at: Option<NaiveDateTime>,
    pub banned_till: Option<NaiveDateTime>,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
