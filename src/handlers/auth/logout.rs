use crate::{
    dto::{auth::internal::AccessToken, response::success::Response},
    enums::errors::response::{to_response, ResponseError},
    services::device_service,
    AppState,
};
use axum::{extract::State, http::StatusCode};
use tracing::{error, info};

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
) -> Result<Response<String>, ResponseError> {
    let _ = device_service::logout_device(&state.pool, &access_token.device_id)
        .await
        .map_err(|e| {
            error!("Failed to logout device: {}", e);
            e
        })
        .map_err(to_response)?;

    info!("Device logged out: {:?}", access_token.device_id);
    Ok(Response::new(StatusCode::OK, "Logged out successfully"))
}
