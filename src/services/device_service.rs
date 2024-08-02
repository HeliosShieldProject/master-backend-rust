use crate::{
    data::{enums::DeviceStatus, models::Device, schema},
    dto::device::internal::NewDevice,
    enums::errors::internal::InternalError,
};
use diesel::prelude::*;
use diesel::QueryDsl;
use tracing::info;
use uuid::Uuid;

pub async fn get_device(
    pool: &deadpool_diesel::postgres::Pool,
    device: &NewDevice,
) -> Result<Device, InternalError> {
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

pub async fn add_device(
    pool: &deadpool_diesel::postgres::Pool,
    new_device: &NewDevice,
) -> Result<Device, InternalError> {
    let conn = pool.get().await?;
    let new_device = new_device.clone();

    if let Ok(device) = get_device(&pool, &new_device).await {
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

pub async fn logout_device(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
) -> Result<(), InternalError> {
    let conn = pool.get().await?;
    let device_id = device_id.clone();

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

pub async fn get_devices(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
) -> Result<Vec<Device>, InternalError> {
    let conn = pool.get().await?;
    let user_id = user_id.clone();

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
