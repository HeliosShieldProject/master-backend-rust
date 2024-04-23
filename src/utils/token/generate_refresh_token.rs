use crate::dto::auth::RefreshToken;
use crate::{config::ENV, enums::AuthError};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

pub async fn generate_refresh_token(user_id: &str, device_id: &str) -> Result<String, AuthError> {
    let now = Local::now();
    let refresh_token_values = RefreshToken {
        exp: (now + Duration::days(7)).timestamp(),
        iat: now.timestamp(),
        user_id: Uuid::parse_str(&user_id).map_err(|_| AuthError::TokenCreation)?,
        device_id: Uuid::parse_str(&device_id).map_err(|_| AuthError::TokenCreation)?,
    };

    let refresh_token = encode(
        &Header::default(),
        &refresh_token_values,
        &EncodingKey::from_secret(ENV.jwt_refresh_secret.as_ref()),
    )
    .map_err(|_| AuthError::TokenCreation)?;

    Ok(refresh_token)
}
