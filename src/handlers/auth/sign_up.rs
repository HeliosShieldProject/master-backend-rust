use crate::{
    config::ENV,
    data::{
        enums,
        repositories::{device_repository, user_repository},
    },
    utils::hash,
    AppState,
};
use axum::{extract::State, http::StatusCode, response, Json};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;

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

#[derive(Debug, Serialize)]
pub struct Response {
    access_token: String,
    refresh_token: String,
}

impl Response {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}

pub async fn sign_up(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, AuthError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    if payload.device.os.is_empty() || payload.device.name.is_empty() {
        return Err(AuthError::MissingDevice);
    }

    let existing_user = user_repository::get_by_email(&state.pool, &payload.email).await;
    if existing_user.is_ok() {
        return Err(AuthError::UserAlreadyExists);
    }

    let hashed_password =
        hash::hash_password(&payload.password).map_err(|_| AuthError::TokenCreation)?;
    let new_user = user_repository::NewUser {
        email: payload.email,
        password: hashed_password,
    };
    let user = user_repository::add_user(&state.pool, new_user).await;
    if user.is_err() {
        return Err(AuthError::WrongEmail);
    }
    let user = user.unwrap();

    let device = device_repository::NewDevice {
        name: payload.device.name,
        os: enums::OS::from_str(&payload.device.os),
        user_id: user.id,
    };

    let device = device_repository::add_device(&state.pool, device).await;
    if device.is_err() {
        return Err(AuthError::MissingDevice);
    }
    let device = device.unwrap();

    let now = Local::now();
    let iat = now.timestamp();
    let access_exp = (now + Duration::hours(1)).timestamp();
    let refresh_exp = (now + Duration::days(7)).timestamp();

    let access_claims = Claims {
        device_id: device.id.to_string(),
        user_id: user.id.to_string(),
        exp: access_exp,
        iat,
    };
    let refresh_claims = Claims {
        device_id: device.id.to_string(),
        user_id: user.id.to_string(),
        exp: refresh_exp,
        iat,
    };

    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(ENV.jwt_access_secret.as_ref()),
    )
    .map_err(|_| AuthError::TokenCreation)?;
    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(ENV.jwt_refresh_secret.as_ref()),
    )
    .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(Response::new(access_token, refresh_token)))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: i64,
    iat: i64,
    device_id: String,
    user_id: String,
}

#[derive(Debug)]
pub enum AuthError {
    UserAlreadyExists,
    WrongEmail,
    MissingCredentials,
    MissingDevice,
    TokenCreation,
}

impl response::IntoResponse for AuthError {
    fn into_response(self) -> response::Response {
        let (status, error_message) = match self {
            AuthError::UserAlreadyExists => (StatusCode::BAD_REQUEST, "User already exists"),
            AuthError::WrongEmail => (StatusCode::BAD_REQUEST, "Wrong email"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::MissingDevice => (StatusCode::BAD_REQUEST, "Missing device"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
