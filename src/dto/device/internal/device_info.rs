use serde::{Deserialize, Serialize};

use crate::data::enums::OS;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    pub os: OS,
    pub name: String,
}
