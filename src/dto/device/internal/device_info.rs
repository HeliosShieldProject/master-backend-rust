use crate::data::enums::OS;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DeviceInfo {
    #[schema(example = "Android")]
    pub os: OS,
    #[schema(example = "Vitya Phone")]
    pub name: String,
}
