use crate::{
    dto::{
        auth::{internal::NewUser, request::SignUpRequest, response::Tokens},
        device::internal::DeviceInfo,
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
    post,
    path = "/auth/sign-up",
    responses(
        (
            status = 201,
            description = "Signed up successfully", 
            body = Tokens,
            example = json!({
                "message": "Signed up successfully",
                "data": {
                    "access_token": "access",
                    "refresh_token": "refresh"
                }
            })
        ),
        (
            status = 409,
            description = "User already exists",
            body = (),
            example = json!({
                "message": "User already exists",
                "error": "UserExists "
            })
        ),
        (
            status = 400,
            description = "Missing credentials or device",
            body = (),
            example = json!({
                "message": "Missing credentials | Missing device",
                "error": "MissingCredentials | MissingDevice"
            })
        ),
    )
)]
pub async fn sign_up(
    State(state): State<AppState>,
    Json(payload): Json<SignUpRequest>,
) -> Result<Response<Tokens>, ExternalError> {
    let tokens = user_service::sign_up(
        &state.pool,
        &NewUser {
            email: payload.email.clone(),
            password: payload.password,
        },
        &DeviceInfo {
            os: payload.device.os,
            name: payload.device.name,
        },
    )
    .await?;

    info!("User signed up successfully: {:?}", payload.email);

    Ok(Response::new(StatusCode::CREATED, "Signed up successfully").with_data(tokens))
}
