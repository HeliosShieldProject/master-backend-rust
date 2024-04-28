use crate::{
    dto::{
        auth::{internal::NewUser, request::SignInRequest, response::Tokens},
        device::internal::DeviceInfo,
    },
    enums::errors::response::{to_response, ResponseError},
    services::user_service,
    AppState,
};
use axum::{extract::State, Json};

pub async fn sign_in(
    State(state): State<AppState>,
    Json(payload): Json<SignInRequest>,
) -> Result<Json<Tokens>, ResponseError> {
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

    Ok(Json(tokens))
}
