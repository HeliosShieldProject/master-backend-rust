use crate::{
    dto::{
        auth::{internal::NewUser, request::SignInRequest, response::Tokens},
        device::internal::DeviceInfo,
        response::success::Response,
    },
    enums::errors::response::{to_response, ResponseError},
    services::user_service,
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};
use tracing::{error, info};

#[utoipa::path(
    tag = "Auth",
    post,
    path = "/auth/sign-in",
    responses(
        (
            status = 200,
            description = "Signed in successfully", 
            body = Tokens,
            example = json!({
                "message": "Signed in successfully",
                "data": {
                    "access_token": "access",
                    "refresh_token": "refresh"
                }
            })
        ),
        (
            status = 404,
            description = "User not found",
            body = (),
            example = json!({
                "message": "User not found",
                "error": "UserNotFound"
            })
        ),
        (
            status = 401,
            description = "Wrong password",
            body = (),
            example = json!({
                "message": "Wrong password",
                "error": "WrongPassword"
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
pub async fn sign_in(
    State(state): State<AppState>,
    Json(payload): Json<SignInRequest>,
) -> Result<Response<Tokens>, ResponseError> {
    let tokens = user_service::sign_in(
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
    .await
    .map_err(|e| {
        error!("Failed to sign in: {}", e);
        e
    })
    .map_err(to_response)?;

    info!("User signed in: {}", payload.email);
    Ok(Response::new(StatusCode::OK, "Signed in successfully").with_data(tokens))
}
