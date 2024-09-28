use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    data::{
        enums::{Country, SessionStatus},
        models::Session,
        schema,
    },
    dto::session::{internal::SessionHistory, request::Params},
    enums::errors::internal::Result,
};

pub async fn get_history(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
    params: &Params,
) -> Result<Vec<SessionHistory>> {
    todo!()
}
