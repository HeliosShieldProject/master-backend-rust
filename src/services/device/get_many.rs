use diesel::{prelude::*, QueryDsl};
use tracing::info;
use uuid::Uuid;

use crate::{
    data::{models::Device, schema},
    enums::errors::internal::Result,
};

pub async fn get_many(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
) -> Result<Vec<Device>> {
    let conn = pool.get().await?;
    let user_id = *user_id;

    let devices = conn
        .interact(move |conn| {
            schema::device::table
                .filter(schema::device::user_id.eq(user_id))
                .select(Device::as_select())
                .load(conn)
        })
        .await??;

    info!("Got devices for user: {}", user_id);

    Ok(devices)
}
