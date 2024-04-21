use crate::{
    config::ENV,
    data::{
        enums,
        repositories::{device_repository, user_repository},
    },
    utils::hash::verify_password,
    AppState,
};
use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response, Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
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

pub async fn refresh(claims: Claims) -> Result<Json<Response>, AuthError> {
    let now = Local::now();
    let iat = now.timestamp();
    let access_exp = (now + Duration::hours(1)).timestamp();
    let refresh_exp = (now + Duration::days(7)).timestamp();

    let access_claims = Claims {
        device_id: claims.device_id.clone(),
        user_id: claims.user_id.clone(),
        exp: access_exp,
        iat,
    };
    let refresh_claims = Claims {
        device_id: claims.device_id.clone(),
        user_id: claims.user_id.clone(),
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
pub struct Claims {
    exp: i64,
    iat: i64,
    device_id: String,
    user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::WrongEmail)?;
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(ENV.jwt_refresh_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::WrongPassword)?;

        Ok(token_data.claims)
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongPassword,
    WrongEmail,
    MissingCredentials,
    MissingDevice,
    TokenCreation,
}

impl response::IntoResponse for AuthError {
    fn into_response(self) -> response::Response {
        let (status, error_message) = match self {
            AuthError::WrongPassword => (StatusCode::BAD_REQUEST, "Wrong password"),
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
