use serde::Deserialize;
use crate::dto::device::internal::DeviceInfo;

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
    pub device: DeviceInfo,
}