use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    pub os: String,
    pub name: String,
}