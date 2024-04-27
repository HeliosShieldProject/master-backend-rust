use crate::{
    data::{
        enums,
        repositories::{device_repository, user_repository},
    },
    dto::{auth::response::Tokens, device},
    enums::errors::response::{to_response, ResponseError},
    utils::{hash::verify_password, token::generate_tokens},
    AppState,
};
use axum::{extract::State, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Device {
    os: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Request {
    email: String,
    password: String,
    device: Device,
}

pub async fn sign_in(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<Json<Tokens>, ResponseError> {
    let user = user_repository::get_by_email(&state.pool, &payload.email)
        .await
        .map_err(to_response)?;

    verify_password(&payload.password, &user.password).map_err(to_response)?;

    let device = device::internal::NewDevice {
        name: payload.device.name.clone(),
        os: enums::OS::from_str(&payload.device.os),
        user_id: user.id.clone(),
    };

    let device = device_repository::add_device(&state.pool, &device)
        .await
        .map_err(to_response)?;

    let (access_token, refresh_token) =
        generate_tokens(&user.id.to_string(), &device.id.to_string())
            .await
            .map_err(to_response)?;

    Ok(Json(Tokens {access_token, refresh_token }))
}
