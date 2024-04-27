use crate::{
    data::{enums::Country, repositories::session_repository},
    dto::auth::internal::AccessToken,
    enums::errors::response::{to_response, ResponseError},
    AppState,
};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Request {
    country: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub session_id: Uuid,
    pub server_public_key: String,
    pub wireguard_uri: String,
    pub user_ip: String,
    pub user_private_key: String,
}

pub async fn create_session(
    claims: AccessToken,
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, ResponseError> {
    let country = Country::from_str(&payload.country).map_err(to_response)?;
    let session = session_repository::create_session(&state.pool, &claims.device_id, &country)
        .await
        .map_err(to_response)?;
    Ok(Json(session))
}
