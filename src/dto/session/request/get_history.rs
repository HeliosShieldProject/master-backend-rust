use serde::Deserialize;
use uuid::Uuid;

use crate::data::enums::Country;

#[derive(Deserialize, Debug, Clone)]
pub struct Params {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub countries: Option<Vec<Country>>,
    pub devices: Option<Vec<Uuid>>,
}
