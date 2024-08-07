use crate::data::enums::Country;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct CreateSession {
    pub country: Country,
}
