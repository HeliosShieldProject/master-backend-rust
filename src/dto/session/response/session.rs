use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::models;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub session_id: Uuid,
    pub link: String,
}

impl From<models::Session> for Session {
    fn from(session: models::Session) -> Self {
        Session {
            session_id: session.id,
            link: session.link,
        }
    }
}
