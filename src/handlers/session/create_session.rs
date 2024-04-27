use crate::{
    data::{enums::Country, repositories::session_repository},
    dto::{
        auth::internal::AccessToken,
        session::{request, response},
    },
    enums::errors::response::{to_response, ResponseError},
    AppState,
};
use axum::{extract::State, Json};

pub async fn create_session(
    claims: AccessToken,
    State(state): State<AppState>,
    Json(payload): Json<request::CreateSession>,
) -> Result<Json<response::Session>, ResponseError> {
    let country = Country::from_str(&payload.country).map_err(to_response)?;
    let session = session_repository::create_session(&state.pool, &claims.device_id, &country)
        .await
        .map_err(to_response)?;
    Ok(Json(session))
}
