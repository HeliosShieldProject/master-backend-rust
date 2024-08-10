use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use tracing::info;

use crate::{
    dto::{
        auth::{internal::NewUser, request::SignInRequest, response::Tokens},
        device::internal::DeviceInfo,
        response::success::Response,
    },
    enums::errors::external::Result,
    extractors::Json,
    services::user,
};

pub async fn sign_in(
    State(pool): State<Pool>,
    Json(payload): Json<SignInRequest>,
) -> Result<Response<Tokens>> {
    let tokens = user::sign_in(
        &pool,
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

    info!("User signed in: {}", payload.email);

    Ok(Response::new(StatusCode::OK, "Signed in successfully").with_data(tokens))
}
