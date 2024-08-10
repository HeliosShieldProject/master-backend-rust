use serde::{Deserialize, Serialize};

use crate::{data::enums::OAuthProvider, dto::device::internal::DeviceInfo};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthorizeRequest {
    pub code: String,
    pub provider: OAuthProvider,
    pub device: DeviceInfo,
}
