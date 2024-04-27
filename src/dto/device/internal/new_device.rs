use crate::data::{enums::OS, schema};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable, Clone)]
#[diesel(table_name = schema::device)]
pub struct NewDevice {
    pub name: String,
    pub os: OS,
    pub user_id: Uuid,
}
