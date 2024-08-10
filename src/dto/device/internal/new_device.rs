use diesel::prelude::*;
use uuid::Uuid;

use crate::data::{enums::OS, schema};

#[derive(Insertable, Clone)]
#[diesel(table_name = schema::device)]
pub struct NewDevice {
    pub name: String,
    pub os: OS,
    pub user_id: Uuid,
}
