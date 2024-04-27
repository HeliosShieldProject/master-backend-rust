use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateSession {
    pub country: String,
}