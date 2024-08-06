use serde::{Deserialize, Serialize};

use crate::data::models::{ClassicAuth, OAuth, User};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullUser {
    pub user: User,
    pub oauth: Option<Vec<OAuth>>,
    pub classic_auth: Option<ClassicAuth>,
}
