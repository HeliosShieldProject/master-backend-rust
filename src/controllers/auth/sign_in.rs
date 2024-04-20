use axum::{extract::State, http::StatusCode, response, Json};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::uuid;

use crate::{data::repositories::user_repository, AppState};

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

pub async fn sign_in(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, AuthError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    if payload.device.os.is_empty() || payload.device.name.is_empty() {
        return Err(AuthError::MissingDevice);
    }
    let user_id = uuid!("4179ac9e-1fb1-4ab6-8aee-005a8670462a");
    let user = user_repository::get_by_id(&state.pool, user_id).await;
    if user.is_err() {
        return Err(AuthError::WrongCredentials);
    }
    let user = user.unwrap();

    let now = Local::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(1)).timestamp();
    let claims = Claims {
        device_id: "someId".to_string(),
        user_id: user.id.to_string(),
        exp,
        iat,
    };
    let access_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.env.jwt_access_secret().as_ref()),
    )
    .map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(Response::new(
        access_token,
        "refresh_token".to_string(),
    )))
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
    WrongCredentials,
    MissingCredentials,
    MissingDevice,
    TokenCreation,
}

impl response::IntoResponse for AuthError {
    fn into_response(self) -> response::Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
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
