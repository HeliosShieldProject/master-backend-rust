use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::data::enums::OS;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct DeviceInfo {
    #[schema(example = "Android")]
    pub os: OS,
    #[schema(example = "Vitya Phone")]
    pub name: String,
}
