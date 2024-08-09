use crate::{
    dto::{
        auth::internal::AccessToken,
        response::success::Response,
        session::{request::CreateSession, response::Session},
    },
    enums::errors::external::ExternalError,
    extractors::Json,
    services::session_service,
};
use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use tracing::info;

#[utoipa::path(
    tag = "Session",
    post,
    path = "/session",
    security(
        ("access_token" = ["Bearer"])
    ),
    responses(
        (
            status = 201,
            description = "Session created successfully",
            body = Session,
            example = json!({
                "message": "Session created successfully",
                "data": {
                    "session_id": "some-uuid",
                    "server_public_key": "some-private-key",
                    "wireguard_uri": "some-uri",
                    "user_ip": "some-ip",
                    "user_private_key": "some-public-key",
                }
            })
        ),
        (
            status = 400,
            description = "Invalid country",
            body = (),
            example = json!({
                "message": "Invalid country",
                "error": "InvalidCountry"
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
        )
    )
)]
pub async fn create_session(
    claims: AccessToken,
    State(pool): State<Pool>,
    Json(payload): Json<CreateSession>,
) -> Result<Response<Session>, ExternalError> {
    let session =
        session_service::create_session(&pool, &claims.device_id, &payload.country).await?;

    info!("Session created successfully: {}", session.session_id);

    Ok(Response::new(StatusCode::CREATED, "Session created successfully").with_data(session))
}
