use crate::{data::enums::{DeviceStatus, OS}, dto::device};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Serialize)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub os: OS,
    pub status: DeviceStatus,
}

impl From<device::Device> for Device {
    fn from(device: device::Device) -> Self {
        Self {
            id: device.id,
            name: device.name,
            os: device.os,
            status: device.status,
        }
    }
}