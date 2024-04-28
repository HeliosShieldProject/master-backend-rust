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
