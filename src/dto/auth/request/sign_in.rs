use serde::{Deserialize, Serialize};

use crate::dto::device::internal::DeviceInfo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
    pub device: DeviceInfo,
}
