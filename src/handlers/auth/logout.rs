use crate::{
    dto::{auth::internal::AccessToken, response::success::Response},
    enums::errors::external::ExternalError,
    services::device_service,
};
use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use tracing::info;

#[utoipa::path(
    tag = "Auth",
    post,
    path = "/auth/logout",
    security(
        ("access_token" = ["Bearer"])
    ),
)]
pub async fn logout(
    State(pool): State<Pool>,
    access_token: AccessToken,
) -> Result<Response<String>, ExternalError> {
    device_service::logout_device(&pool, &access_token.device_id).await?;

    info!("Device logged out: {:?}", access_token.device_id);

    Ok(Response::new(StatusCode::OK, "Logged out successfully"))
}
