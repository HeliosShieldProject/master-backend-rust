use crate::{
    data::enums::Country,
    dto::{
        auth::internal::AccessToken,
        response::success::SuccessResponse,
        session::{request::CreateSession, response::Session},
    },
    enums::errors::response::{to_response, ResponseError},
    logger::{enums::Handlers, ResultExtReponse},
    services::session_service,
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};

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
    State(state): State<AppState>,
    Json(payload): Json<CreateSession>,
) -> Result<SuccessResponse<Session>, ResponseError> {
    let country = Country::from_str(&payload.country).map_err(to_response)?;
    let session = session_service::create_session(&state.pool, &claims.device_id, &country)
        .await
        .map_err(to_response)
        .log_error(Handlers::CreateSession)
        .await?;

    Ok(
        SuccessResponse::new(StatusCode::CREATED, "Session created successfully")
            .with_data(session),
    )
}
