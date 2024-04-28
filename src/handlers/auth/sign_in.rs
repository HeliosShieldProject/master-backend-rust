use crate::{
    dto::{
        auth::{internal::NewUser, request::SignInRequest, response::Tokens},
        device::internal::DeviceInfo,
        response::success::SuccessResponse,
    },
    enums::errors::response::{to_response, ResponseError},
    services::user_service,
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};

pub async fn sign_in(
    State(state): State<AppState>,
    Json(payload): Json<SignInRequest>,
) -> Result<SuccessResponse<Tokens>, ResponseError> {
    let tokens = user_service::sign_in(
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

    Ok(SuccessResponse::new(StatusCode::OK, "Signed in successfully").with_data(tokens))
}
