use axum::{extract::State, http::StatusCode};
use tracing::info;

use crate::{
    dto::{
        auth::{internal::NewUser, request::SignUpRequest, response::Tokens},
        device::internal::DeviceInfo,
        response::success::Response,
    },
    enums::errors::external::Result,
    extractors::Json,
    services::user,
    state::AppState,
};

pub async fn sign_up(
    State(state): State<AppState>,
    Json(payload): Json<SignUpRequest>,
) -> Result<Response<Tokens>> {
    let tokens = user::sign_up(
        state,
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
