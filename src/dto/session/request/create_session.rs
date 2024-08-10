use crate::data::enums::Country;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSession {
    pub country: Country,
}
