use serde::{Deserialize, Serialize};

use crate::data::enums::Country;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSession {
    pub country: Country,
}
