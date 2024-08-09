use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::enums::Country;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionHistory {
    pub id: Uuid,
    pub device_id: Uuid,
    pub opened_at: NaiveDateTime,
    pub closed_at: NaiveDateTime,
    pub duration: i64,
    pub country: Country,
}
