use crate::{data::enums::OAuthProvider, dto::device::internal::DeviceInfo};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct AuthorizeRequest {
    pub code: String,
    pub provider: OAuthProvider,
    pub device: DeviceInfo,
}
