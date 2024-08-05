use crate::{
    dto::{auth::internal::AccessToken, response::success::Response},
    enums::errors::external::ExternalError,
    services::session_service,
};
use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use tracing::info;

#[utoipa::path(
    tag = "Session",
    put,
    path = "/session",
    security(
        ("access_token" = ["Bearer"])
    ),
    responses(
        (
            status = 200,
            description = "Closed session successfully",
            body = (),
            example = json!({
                "message": "Closed session successfully"
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
            status = 404,
            description = "Session not found",
            body = (),
            example = json!({
                "message": "Session not found",
                "error": "SessionNotFound"
            })
        )
    )
)]
pub async fn close_session(
    claims: AccessToken,
    State(pool): State<Pool>,
) -> Result<Response<String>, ExternalError> {
    let session_id = session_service::close_session(&pool, &claims.device_id).await?;

    info!("Closed session successfully: {}", session_id);

    Ok(Response::new(StatusCode::OK, "Closed session successfully"))
}
