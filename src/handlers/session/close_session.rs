use crate::{
    data::repositories::session_repository,
    dto::auth::internal::AccessToken,
    enums::errors::response::{to_response, ResponseError},
    AppState,
};
use axum::extract::State;

pub async fn close_session(
    claims: AccessToken,
    State(state): State<AppState>,
) -> Result<String, ResponseError> {
    session_repository::close_session(&state.pool, &claims.device_id)
        .await
        .map_err(to_response)?;
    Ok("Session closed".to_string())
}
