use crate::dto::device::internal::DeviceInfo;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub device: DeviceInfo,
}
