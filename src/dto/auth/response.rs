use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    pub access_token: String,
    pub refresh_token: String,
}
