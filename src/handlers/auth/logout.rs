use crate::{
    dto::{auth::internal::AccessToken, response::success::SuccessResponse},
    enums::errors::response::{to_response, ResponseError},
    logger::{enums::Handlers::Logout, ResultExtReponse},
    services::device_service,
    AppState,
};
use axum::{extract::State, http::StatusCode};

#[utoipa::path(
    tag = "Auth",
    post,
    path = "/auth/logout",
    security(
        ("access_token" = ["Bearer"])
    ),
)]
pub async fn logout(
    State(state): State<AppState>,
    access_token: AccessToken,
) -> Result<SuccessResponse<String>, ResponseError> {
    let _ = device_service::logout_device(&state.pool, &access_token.device_id)
        .await
        .map_err(to_response)
        .log_error(Logout)
        .await;

    Ok(SuccessResponse::new(
        StatusCode::OK,
        "Logged out successfully",
    ))
}
