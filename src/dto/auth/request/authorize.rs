use crate::{data::enums::OAuthProvider, dto::device::internal::DeviceInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthorizeRequest {
    pub code: String,
    pub provider: OAuthProvider,
    pub device: DeviceInfo,
}
