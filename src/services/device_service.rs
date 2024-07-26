use crate::{
    data::{enums::DeviceStatus, models::Device, schema},
    dto::device::internal::NewDevice,
    enums::errors::internal::{to_internal, DeviceError, InternalError},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use tracing::{error, info};
use uuid::Uuid;

pub async fn get_device(
    pool: &deadpool_diesel::postgres::Pool,
    device: &NewDevice,
) -> Result<Device, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let device = device.clone();

    conn.interact(move |conn| {
        schema::device::table
            .filter(schema::device::name.eq(device.name))
            .filter(schema::device::os.eq(device.os))
            .filter(schema::device::user_id.eq(device.user_id))
            .select(Device::as_select())
            .first(conn)
    })
    .await
    .map_err(|e| {
        error!("Device not found: {}", e);
        e
    })
    .map_err(to_internal)?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => InternalError::DeviceError(DeviceError::DeviceNotFound),
        _ => InternalError::Internal,
    })
    .map(|device| {
        info!("Got device by name: {}", device.id);
        device
    })
}

pub async fn add_device(
    pool: &deadpool_diesel::postgres::Pool,
    new_device: &NewDevice,
) -> Result<Device, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let new_device = new_device.clone();

    if let Ok(device) = get_device(&pool, &new_device).await {
        return conn
            .interact(move |conn| {
                diesel::update(schema::device::table)
                    .filter(schema::device::name.eq(new_device.name))
                    .filter(schema::device::os.eq(new_device.os))
                    .filter(schema::device::user_id.eq(new_device.user_id))
                    .set(schema::device::status.eq(DeviceStatus::LoggedIn))
                    .execute(conn)
            })
            .await
            .map_err(|e| {
                error!("Error updating device: {}", e);
                e
            })
            .map_err(to_internal)?
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    InternalError::DeviceError(DeviceError::DeviceNotFound)
                }
                _ => InternalError::Internal,
            })
            .map(|_| {
                info!("Updated device: {}", device.id);
                device
            });
    }

    conn.interact(move |conn| {
        diesel::insert_into(schema::device::table)
            .values(new_device)
            .get_result(conn)
    })
    .await
    .map_err(|e| {
        error!("Error adding device: {}", e);
        e
    })
    .map_err(to_internal)?
    .map_err(|e| match e {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        ) => InternalError::DeviceError(DeviceError::DeviceAlreadyExists),
        _ => InternalError::Internal,
    })
    .map(|device: Device| {
        info!("Added device: {}", device.id);
        device
    })
}

pub async fn logout_device(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
) -> Result<(), InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let device_id = device_id.clone();

    conn.interact(move |conn| {
        diesel::update(schema::device::table)
            .filter(schema::device::id.eq(device_id))
            .set(schema::device::status.eq(DeviceStatus::LoggedOut))
            .execute(conn)
    })
    .await
    .map_err(|e| {
        error!("Error logging out device: {}", e);
        e
    })
    .map_err(to_internal)
    .map(|_| {
        info!("Logged out device: {}", device_id);
    })
}

pub async fn get_devices(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
) -> Result<Vec<Device>, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let user_id = user_id.clone();

    conn.interact(move |conn| {
        schema::device::table
            .filter(schema::device::user_id.eq(user_id))
            .select(Device::as_select())
            .load(conn)
    })
    .await
    .map_err(|e| {
        error!("Error getting devices: {}", e);
        e
    })
    .map_err(to_internal)?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => InternalError::DeviceError(DeviceError::DeviceNotFound),
        _ => InternalError::Internal,
    })
    .map(|devices: Vec<Device>| {
        let ids = devices
            .iter()
            .map(|device| device.id)
            .collect::<Vec<Uuid>>();
        info!("Got devices: {:?}", ids);
        devices
    })
}
