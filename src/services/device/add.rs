use diesel::prelude::*;
use tracing::info;

use super::get;
use crate::{
    data::{enums::DeviceStatus, models::Device, schema},
    dto::device::internal::NewDevice,
    enums::errors::internal::Result,
};

pub async fn add(pool: &deadpool_diesel::postgres::Pool, new_device: &NewDevice) -> Result<Device> {
    let conn = pool.get().await?;
    let new_device = new_device.clone();

    if let Ok(device) = get(pool, &new_device).await {
        conn.interact(move |conn| {
            diesel::update(schema::device::table)
                .filter(schema::device::name.eq(new_device.name))
                .filter(schema::device::os.eq(new_device.os))
                .filter(schema::device::user_id.eq(new_device.user_id))
                .set(schema::device::status.eq(DeviceStatus::LoggedIn))
                .execute(conn)
        })
        .await??;

        info!("Updated device: {}", device.id);

        return Ok(device);
    }

    let device: Device = conn
        .interact(move |conn| {
            diesel::insert_into(schema::device::table)
                .values(new_device)
                .get_result(conn)
        })
        .await??;

    info!("Added device: {}", device.id);

    Ok(device)
}
