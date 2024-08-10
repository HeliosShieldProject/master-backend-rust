use crate::dto::device::internal::DeviceInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub device: DeviceInfo,
}
