use crate::{
    dto::{
        auth::{internal::NewUser, request::SignUpRequest, response::Tokens},
        device::internal::DeviceInfo,
        response::success::SuccessResponse,
    },
    enums::errors::response::{to_response, ResponseError},
    services::user_service,
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};

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
            description = "User already exists or password is the same",
            body = (),
            example = json!({
                "message": "User already exists | Password is the same",
                "error": "UserExists | PasswordIsSame"
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
) -> Result<SuccessResponse<Tokens>, ResponseError> {
    let tokens = user_service::sign_up(
        &state.pool,
        &NewUser {
            email: payload.email,
            password: payload.password,
        },
        &DeviceInfo {
            os: payload.device.os,
            name: payload.device.name,
        },
    )
    .await
    .map_err(to_response)?;

    Ok(SuccessResponse::new(StatusCode::CREATED, "Signed up successfully").with_data(tokens))
}
