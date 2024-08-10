use diesel::prelude::*;
use tracing::info;
use uuid::Uuid;

use crate::{
    data::{enums::DeviceStatus, schema},
    enums::errors::internal::Result,
};

pub async fn logout(pool: &deadpool_diesel::postgres::Pool, device_id: &Uuid) -> Result<()> {
    let conn = pool.get().await?;
    let device_id = *device_id;

    conn.interact(move |conn| {
        diesel::update(schema::device::table)
            .filter(schema::device::id.eq(device_id))
            .set(schema::device::status.eq(DeviceStatus::LoggedOut))
            .execute(conn)
    })
    .await??;

    info!("Logged out device: {}", device_id);

    Ok(())
}
