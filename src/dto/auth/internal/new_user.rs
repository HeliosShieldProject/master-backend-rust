use crate::data::schema;
use diesel::prelude::*;

#[derive(Insertable, Clone)]
#[diesel(table_name = schema::user)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}
