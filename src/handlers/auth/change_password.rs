use crate::{
    dto::{
        auth::{internal::AccessToken, request::ChangePasswordRequest},
        response::success::SuccessResponse,
    },
    enums::errors::response::{to_response, ResponseError},
    services::user_service,
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};

pub async fn change_password(
    claims: AccessToken,
    State(state): State<AppState>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<SuccessResponse<String>, ResponseError> {
    user_service::change_password(&state.pool, &claims.user_id, &payload.password)
        .await
        .map_err(to_response)?;

    Ok(SuccessResponse::new(
        StatusCode::OK,
        "Password changed successfully",
    ))
}
