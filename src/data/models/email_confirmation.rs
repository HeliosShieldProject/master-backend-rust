use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use uuid::Uuid;

use crate::data::schema;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::email_confirmation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct EmailConfirmation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub confirmed: bool,
    pub confirmed_at: Option<NaiveDateTime>,
}
