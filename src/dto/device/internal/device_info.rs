use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DeviceInfo {
    #[schema(example = "Android")]
    pub os: String,
    #[schema(example = "Vitya Phone")]
    pub name: String,
}
