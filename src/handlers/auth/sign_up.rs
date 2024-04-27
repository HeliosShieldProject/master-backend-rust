use crate::{
    data::{
        enums,
        repositories::{device_repository, user_repository},
    },
    dto::{auth, auth::response::Tokens, device},
    enums::errors::response::{to_response, AuthError, ResponseError},
    utils::{hash, token::generate_tokens},
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

pub async fn sign_up(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<Json<Tokens>, ResponseError> {
    if user_repository::get_by_email(&state.pool, &payload.email)
        .await
        .is_ok()
    {
        return Err(ResponseError::AuthError(AuthError::UserAlreadyExists));
    }

    let hashed_password = hash::hash_password(&payload.password).map_err(to_response)?;

    let new_user = auth::internal::NewUser {
        email: payload.email.clone(),
        password: hashed_password.clone(),
    };

    let user = user_repository::add_user(&state.pool, &new_user)
        .await
        .map_err(to_response)?;

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

    Ok(Json(Tokens {
        access_token,
        refresh_token,
    }))
}
