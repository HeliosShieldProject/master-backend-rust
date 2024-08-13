use diesel::{prelude::*, QueryDsl};
use tracing::info;
use uuid::Uuid;

use crate::{
    data::{models::Device, schema},
    enums::errors::internal::Result,
};

pub async fn get_by_id(pool: &deadpool_diesel::postgres::Pool, device_id: &Uuid) -> Result<Device> {
    let conn = pool.get().await?;
    let device_id = *device_id;

    let device = conn
        .interact(move |conn| {
            schema::device::table
                .filter(schema::device::id.eq(device_id))
                .first::<Device>(conn)
        })
        .await??;

    info!("Got device: {}", device.id);

    Ok(device)
}
