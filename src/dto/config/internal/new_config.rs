use crate::data::schema;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Insertable, Clone)]
#[diesel(table_name = schema::Config)]
pub struct NewConfig {
    pub private_key: String,
    pub user_ip: String,
    pub server_id: Uuid,
}
