use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

use crate::data::{
    enums::{DeviceStatus, OS},
    schema,
};

#[derive(Queryable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = schema::device)]
#[diesel(belongs_to(crate::dto::user::User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub os: OS,
    pub user_id: Uuid,
    pub banned_at: Option<NaiveDateTime>,
    pub banned_till: Option<NaiveDateTime>,
    pub revoked_at: Option<NaiveDateTime>,
    pub status: DeviceStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
