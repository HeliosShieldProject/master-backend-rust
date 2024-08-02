use crate::{
    dto::{
        auth::{internal::AccessToken, request::ChangePasswordRequest},
        response::success::Response,
    },
    enums::errors::external::ExternalError,
    services::user_service,
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};
use tracing::info;

#[utoipa::path(
    tag = "Auth",
    put,
    path = "/auth/change-password",
    security(
        ("access_token" = ["Bearer"])
    ),
    responses(
        (
            status = 200,
            description = "Password changed successfully",
            body = (),
            example = json!({
                "message": "Password changed successfully"
            })
        ),
        (
            status = 401,
            description = "Wrong token",
            body = (),
            example = json!({
                "message": "Wrong token",
                "error": "WrongToken"
            })
        ),
        (
            status = 400,
            description = "Missing credentials",
            body = (),
            example = json!({
                "message": "Missing credentials",
                "error": "MissingCredentials"
            })
        ),
    )
)]
pub async fn change_password(
    claims: AccessToken,
    State(state): State<AppState>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Response<String>, ExternalError> {
    user_service::change_password(&state.pool, &claims.user_id, &payload.password).await?;

    info!("Password changed successfully for user: {}", claims.user_id);

    Ok(Response::new(
        StatusCode::OK,
        "Password changed successfully",
    ))
}
