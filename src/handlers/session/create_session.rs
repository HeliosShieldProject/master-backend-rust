use crate::{
    data::enums::Country,
    dto::{
        auth::internal::AccessToken,
        session::{request, response},
    },
    enums::errors::response::{to_response, ResponseError},
    services::session_service,
    AppState,
};
use axum::{extract::State, Json};

pub async fn create_session(
    claims: AccessToken,
    State(state): State<AppState>,
    Json(payload): Json<request::CreateSession>,
) -> Result<Json<response::Session>, ResponseError> {
    let country = Country::from_str(&payload.country).map_err(to_response)?;
    let session = session_service::create_session(&state.pool, &claims.device_id, &country)
        .await
        .map_err(to_response)?;
    Ok(Json(session))
}
