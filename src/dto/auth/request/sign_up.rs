use crate::dto::device::internal::DeviceInfo;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct SignUpRequest {
    #[schema(example = "vitya@gmail.com")]
    pub email: String,
    #[schema(example = "strong_password")]
    pub password: String,
    pub device: DeviceInfo,
}
