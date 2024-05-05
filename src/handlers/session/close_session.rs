use crate::{
    dto::{auth::internal::AccessToken, response::success::SuccessResponse},
    enums::errors::response::{to_response, ResponseError},
    logger::{enums::Handlers::CloseSession, ResultExtReponse},
    services::session_service,
    AppState,
};
use axum::{extract::State, http::StatusCode};

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
    State(state): State<AppState>,
) -> Result<SuccessResponse<String>, ResponseError> {
    session_service::close_session(&state.pool, &claims.device_id)
        .await
        .map_err(to_response)
        .log_error(CloseSession)
        .await?;

    Ok(SuccessResponse::new(
        StatusCode::OK,
        "Closed session successfully",
    ))
}
