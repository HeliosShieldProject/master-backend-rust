use serde::{Deserialize, Serialize};

use crate::data::enums::{Country, Protocol};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSession {
    pub country: Country,
    pub protocol: Protocol,
}
