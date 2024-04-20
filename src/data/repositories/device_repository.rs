use crate::data::{
    enums::{DeviceStatus, OS},
    errors::{adapt_infra_error, InfraError},
    schema,
};
use diesel::prelude::*;
use diesel::{QueryDsl, Queryable, Selectable};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::Device)]
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

pub async fn get_device(
    pool: &deadpool_diesel::postgres::Pool,
    device: NewDevice,
) -> Result<Device, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
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
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(result)
}

pub async fn add_device(
    pool: &deadpool_diesel::postgres::Pool,
    new_device: NewDevice,
) -> Result<Device, InfraError> {
    let device = get_device(&pool, new_device.clone()).await;
    if device.is_ok() {
        return Ok(device.unwrap());
    }

    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(schema::Device::table)
                .values(&new_device)
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(result)
}
