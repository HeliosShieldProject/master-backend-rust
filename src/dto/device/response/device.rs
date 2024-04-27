use crate::data::{
    enums::{DeviceStatus, OS},
    models,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub os: OS,
    pub status: DeviceStatus,
}

impl From<models::Device> for Device {
    fn from(device: models::Device) -> Self {
        Self {
            id: device.id,
            name: device.name,
            os: device.os,
            status: device.status,
        }
    }
}
