use crate::{
    data::{
        enums::{DeviceStatus, OS},
        schema,
    },
    enums::errors::internal::{to_internal, DeviceError, InternalError},
};
use diesel::prelude::*;
use diesel::{QueryDsl, Queryable, Selectable};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::Device)]
#[diesel(belongs_to(super::user_repository::User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub os: OS,
    pub user_id: Uuid,
    pub banned_at: Option<SystemTime>,
    pub banned_till: Option<SystemTime>,
    pub revoked_at: Option<SystemTime>,
    pub status: DeviceStatus,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = schema::Device)]
pub struct NewDevice {
    pub name: String,
    pub os: OS,
    pub user_id: Uuid,
}

pub async fn get_device_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
) -> Result<Device, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let device_id = device_id.clone();
    let result = conn
        .interact(move |conn| {
            schema::Device::table
                .find(device_id)
                .select(Device::as_select())
                .first(conn)
        })
        .await
        .map_err(to_internal)?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                InternalError::DeviceError(DeviceError::DeviceNotFound)
            }
            _ => InternalError::Internal,
        })?;

    Ok(result)
}

pub async fn get_device(
    pool: &deadpool_diesel::postgres::Pool,
    device: &NewDevice,
) -> Result<Device, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let device = device.clone();
    let result = conn
        .interact(move |conn| {
            schema::Device::table
                .filter(schema::Device::name.eq(device.name))
                .filter(schema::Device::os.eq(device.os))
                .filter(schema::Device::user_id.eq(device.user_id))
                .select(Device::as_select())
                .first(conn)
        })
        .await
        .map_err(to_internal)?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                InternalError::DeviceError(DeviceError::DeviceNotFound)
            }
            _ => InternalError::Internal,
        })?;

    Ok(result)
}

pub async fn add_device(
    pool: &deadpool_diesel::postgres::Pool,
    new_device: &NewDevice,
) -> Result<Device, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let new_device = new_device.clone();

    if let Ok(device) = get_device(&pool, &new_device).await {
        let _ = conn
            .interact(move |conn| {
                diesel::update(schema::Device::table)
                    .filter(schema::Device::name.eq(new_device.name))
                    .filter(schema::Device::os.eq(new_device.os))
                    .filter(schema::Device::user_id.eq(new_device.user_id))
                    .set(schema::Device::status.eq(DeviceStatus::LoggedIn))
                    .execute(conn)
            })
            .await
            .map_err(to_internal)?
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    InternalError::DeviceError(DeviceError::DeviceNotFound)
                }
                _ => InternalError::Internal,
            })?;
        return Ok(device);
    }

    let result = conn
        .interact(move |conn| {
            diesel::insert_into(schema::Device::table)
                .values(new_device)
                .get_result(conn)
        })
        .await
        .map_err(to_internal)?
        .map_err(|e| match e {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => InternalError::DeviceError(DeviceError::DeviceAlreadyExists),
            _ => InternalError::Internal,
        })?;

    Ok(result)
}

pub async fn logout_device(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
) -> Result<(), InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let device_id = device_id.clone();
    let _ = conn
        .interact(move |conn| {
            diesel::update(schema::Device::table)
                .filter(schema::Device::id.eq(device_id))
                .set(schema::Device::status.eq(DeviceStatus::LoggedOut))
                .execute(conn)
        })
        .await
        .map_err(to_internal);

    Ok(())
}

pub async fn get_devices(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
) -> Result<Vec<Device>, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let user_id = user_id.clone();
    let result = conn
        .interact(move |conn| {
            schema::Device::table
                .filter(schema::Device::user_id.eq(user_id))
                .select(Device::as_select())
                .load(conn)
        })
        .await
        .map_err(to_internal)?
        .map_err(|_| InternalError::Internal)?;

    Ok(result)
}
