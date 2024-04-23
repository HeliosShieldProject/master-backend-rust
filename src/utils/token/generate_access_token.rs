use crate::dto::auth::AccessToken;
use crate::{config::ENV, enums::AuthError};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

pub async fn generate_access_token(user_id: &str, device_id: &str) -> Result<String, AuthError> {
    let now = Local::now();
    let access_token_values = AccessToken {
        exp: (now + Duration::hours(1)).timestamp(),
        iat: now.timestamp(),
        device_id: Uuid::parse_str(&device_id).map_err(|_| AuthError::TokenCreation)?,
        user_id: Uuid::parse_str(&user_id).map_err(|_| AuthError::TokenCreation)?,
    };

    let access_token = encode(
        &Header::default(),
        &access_token_values,
        &EncodingKey::from_secret(ENV.jwt_access_secret.as_ref()),
    )
    .map_err(|_| AuthError::TokenCreation)?;

    Ok(access_token)
}
