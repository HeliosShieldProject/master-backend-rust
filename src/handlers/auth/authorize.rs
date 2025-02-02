use axum::{extract::State, http::StatusCode};
use tracing::info;

use crate::{
    dto::{
        auth::{internal::OAuthCode, request::AuthorizeRequest, response::Tokens},
        device::internal::DeviceInfo,
        response::success::Response,
    },
    enums::errors::external::Result,
    extractors::Json,
    services::user,
    state::AppState,
};

pub async fn authorize(
    State(state): State<AppState>,
    Json(payload): Json<AuthorizeRequest>,
) -> Result<Response<Tokens>> {
    let tokens = user::authorize(
        &state,
        &OAuthCode {
            code: payload.code.clone(),
            provider: payload.provider,
        },
        &DeviceInfo {
            os: payload.device.os,
            name: payload.device.name,
        },
    )
    .await?;

    info!(
        "User authorized successfully using {:?}: code {}",
        payload.provider,
        payload.code.chars().take(8).collect::<String>()
            + "*".repeat(payload.code.len() - 8).as_str()
    );

    Ok(Response::new(StatusCode::CREATED, "Authorized successfully").with_data(tokens))
}
