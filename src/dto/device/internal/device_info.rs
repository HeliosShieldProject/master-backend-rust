use crate::data::enums::OS;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    pub os: OS,
    pub name: String,
}
