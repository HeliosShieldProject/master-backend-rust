use crate::{
    data::{
        enums,
        repositories::{device_repository, user_repository},
    },
    dto::auth::Response,
    enums::AuthError,
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
) -> Result<Json<Response>, AuthError> {
    let existing_user = user_repository::get_by_email(&state.pool, &payload.email).await;
    if existing_user.is_ok() {
        return Err(AuthError::UserAlreadyExists);
    }

    let hashed_password =
        hash::hash_password(&payload.password).map_err(|_| AuthError::TokenCreation)?;

    let new_user = user_repository::NewUser {
        email: payload.email.clone(),
        password: hashed_password.clone(),
    };

    let user = user_repository::add_user(&state.pool, &new_user)
        .await
        .map_err(|_| AuthError::UserNotFound)?;

    let device = device_repository::NewDevice {
        name: payload.device.name.clone(),
        os: enums::OS::from_str(&payload.device.os),
        user_id: user.id.clone(),
    };

    let device = device_repository::add_device(&state.pool, &device)
        .await
        .map_err(|_| AuthError::MissingDevice)?;

    let (access_token, refresh_token) =
        generate_tokens(&user.id.to_string(), &device.id.to_string())
            .await
            .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(Response::new(access_token, refresh_token)))
}
