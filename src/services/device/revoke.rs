use diesel::prelude::*;
use tracing::info;
use uuid::Uuid;

use crate::{
    data::{enums::DeviceStatus, schema},
    dto::{
        auth::internal::AccessToken,
        session::{interface::get_session, query::ActiveSessionAndDevice},
    },
    enums::errors::internal::{Device, Error, Result},
    services,
};

pub async fn revoke(
    pool: &deadpool_diesel::postgres::Pool,
    author: AccessToken,
    device_id: &Uuid,
) -> Result<()> {
    let conn = pool.get().await?;
    let device_id = *device_id;

    if author.device_id == device_id {
        return Err(Error::Device(Device::SelfRevocation));
    }

    let device = services::device::get_by_id(pool, &device_id)
        .await
        .map_err(|_| Error::Device(Device::NotFound))?;

    if device.user_id != author.user_id {
        return Err(Error::Device(Device::NotFound));
    }

    if device.status == DeviceStatus::LoggedOut {
        return Err(Error::Device(Device::AlreadyRevoked));
    }

    if let Ok((session, _, _, _)) = get_session(pool, ActiveSessionAndDevice { device_id }).await {
        let _ = services::session::close_by_id(pool, &session.id).await?;
    }

    conn.interact(move |conn| {
        diesel::update(schema::device::table)
            .filter(schema::device::id.eq(device_id))
            .set(schema::device::status.eq(DeviceStatus::LoggedOut))
            .execute(conn)
    })
    .await??;

    info!("Device with id {} has been revoked", device_id);

    Ok(())
}
