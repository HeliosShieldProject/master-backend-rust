use diesel::{prelude::*, QueryDsl};
use tracing::info;

use crate::{
    data::{models::Device, schema},
    dto::device::internal::NewDevice,
    enums::errors::internal::Result,
};

pub async fn get(pool: &deadpool_diesel::postgres::Pool, device: &NewDevice) -> Result<Device> {
    let conn = pool.get().await?;
    let device = device.clone();

    let device = conn
        .interact(move |conn| {
            schema::device::table
                .filter(schema::device::name.eq(device.name))
                .filter(schema::device::os.eq(device.os))
                .filter(schema::device::user_id.eq(device.user_id))
                .select(Device::as_select())
                .first(conn)
        })
        .await??;

    info!("Got device: {}", device.id);

    Ok(device)
}
