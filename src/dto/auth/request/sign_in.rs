use crate::dto::device::internal::DeviceInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
    pub device: DeviceInfo,
}
