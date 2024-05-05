use crate::{
    data::{enums::DeviceStatus, models::Device, schema},
    dto::device::internal::NewDevice,
    enums::errors::internal::{to_internal, DeviceError, InternalError},
    logger::{enums::Services::DeviceService, ContextLogger, ResultExt},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use uuid::Uuid;

const LOG: ContextLogger = ContextLogger::new(DeviceService);

pub async fn get_device(
    pool: &deadpool_diesel::postgres::Pool,
    device: &NewDevice,
) -> Result<Device, InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(DeviceService)
        .await?;
    let device = device.clone();
    let result = conn
        .interact(move |conn| {
            schema::device::table
                .filter(schema::device::name.eq(device.name))
                .filter(schema::device::os.eq(device.os))
                .filter(schema::device::user_id.eq(device.user_id))
                .select(Device::as_select())
                .first(conn)
        })
        .await
        .map_err(to_internal)
        .log_error(DeviceService)
        .await?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                InternalError::DeviceError(DeviceError::DeviceNotFound)
            }
            _ => InternalError::Internal,
        })?;

    LOG.info(format!("Got device by name: {}", result.id)).await;
    Ok(result)
}

pub async fn add_device(
    pool: &deadpool_diesel::postgres::Pool,
    new_device: &NewDevice,
) -> Result<Device, InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(DeviceService)
        .await?;
    let new_device = new_device.clone();

    if let Ok(device) = get_device(&pool, &new_device).await {
        let _ = conn
            .interact(move |conn| {
                diesel::update(schema::device::table)
                    .filter(schema::device::name.eq(new_device.name))
                    .filter(schema::device::os.eq(new_device.os))
                    .filter(schema::device::user_id.eq(new_device.user_id))
                    .set(schema::device::status.eq(DeviceStatus::LoggedIn))
                    .execute(conn)
            })
            .await
            .map_err(to_internal)
            .log(format!("Updated device: {}", device.id), DeviceService)
            .await?
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    InternalError::DeviceError(DeviceError::DeviceNotFound)
                }
                _ => InternalError::Internal,
            })?;

        return Ok(device);
    }

    let result: Device = conn
        .interact(move |conn| {
            diesel::insert_into(schema::device::table)
                .values(new_device)
                .get_result(conn)
        })
        .await
        .map_err(to_internal)
        .log_error(DeviceService)
        .await?
        .map_err(|e| match e {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => InternalError::DeviceError(DeviceError::DeviceAlreadyExists),
            _ => InternalError::Internal,
        })?;

    LOG.info(format!("Added device: {}", result.id)).await;
    Ok(result)
}

pub async fn logout_device(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
) -> Result<(), InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(DeviceService)
        .await?;
    let device_id = device_id.clone();
    let _ = conn
        .interact(move |conn| {
            diesel::update(schema::device::table)
                .filter(schema::device::id.eq(device_id))
                .set(schema::device::status.eq(DeviceStatus::LoggedOut))
                .execute(conn)
        })
        .await
        .map_err(to_internal)
        .log_error(DeviceService)
        .await;

    Ok(())
}

pub async fn get_devices(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
) -> Result<Vec<Device>, InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(DeviceService)
        .await?;
    let user_id = user_id.clone();
    let result = conn
        .interact(move |conn| {
            schema::device::table
                .filter(schema::device::user_id.eq(user_id))
                .select(Device::as_select())
                .load(conn)
        })
        .await
        .map_err(to_internal)
        .log(
            format!("Got devices by user_id: {}", user_id),
            DeviceService,
        )
        .await?
        .map_err(|_| InternalError::Internal)?;

    Ok(result)
}
