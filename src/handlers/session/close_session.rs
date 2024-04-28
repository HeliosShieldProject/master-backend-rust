use crate::{
    dto::{auth::internal::AccessToken, response::success::SuccessResponse},
    enums::errors::response::{to_response, ResponseError},
    services::session_service,
    AppState,
};
use axum::{extract::State, http::StatusCode};

pub async fn close_session(
    claims: AccessToken,
    State(state): State<AppState>,
) -> Result<SuccessResponse<String>, ResponseError> {
    session_service::close_session(&state.pool, &claims.device_id)
        .await
        .map_err(to_response)?;

    Ok(SuccessResponse::new(
        StatusCode::OK,
        "Logged out successfully",
    ))
}
