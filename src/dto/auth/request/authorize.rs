use crate::{dto::device::internal::DeviceInfo, enums::oauth_providers::OAuthProvider};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthorizeRequest {
    pub code: String,
    pub provider: OAuthProvider,
    pub device: DeviceInfo,
}
