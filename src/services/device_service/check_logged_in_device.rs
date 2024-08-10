use diesel::{prelude::*, QueryDsl};
use tracing::info;
use uuid::Uuid;

use crate::{
    data::{enums::DeviceStatus, models::Device, schema},
    enums::errors::internal::Result,
};

pub async fn check_logged_in_device(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
) -> Result<bool> {
    let conn = pool.get().await?;
    let device_id = *device_id;

    let device: Device = conn
        .interact(move |conn| {
            schema::device::table
                .find(device_id)
                .select(Device::as_select())
                .first(conn)
        })
        .await??;

    info!("Checked device: {}", device_id);

    Ok(device.status == DeviceStatus::LoggedIn)
}
