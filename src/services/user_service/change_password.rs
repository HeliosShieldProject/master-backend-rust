use uuid::Uuid;

use crate::{data::models::User, enums::errors::internal::Result};

pub async fn change_password(
    _pool: &deadpool_diesel::postgres::Pool,
    _user_id: &Uuid,
    _new_password: &str,
) -> Result<User> {
    todo!()
}
