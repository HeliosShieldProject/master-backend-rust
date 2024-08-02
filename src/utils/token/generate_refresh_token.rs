use crate::config::ENV;
use crate::dto::auth::internal::RefreshToken;
use crate::enums::errors::internal::InternalError;
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use tracing::info;
use uuid::Uuid;

pub async fn generate_refresh_token(
    user_id: &str,
    device_id: &str,
) -> Result<String, InternalError> {
    let now = Local::now();
    let refresh_token_values = RefreshToken {
        exp: (now + Duration::days(7)).timestamp(),
        iat: now.timestamp(),
        user_id: Uuid::parse_str(user_id)?,
        device_id: Uuid::parse_str(device_id)?,
    };

    let refresh_token = encode(
        &Header::default(),
        &refresh_token_values,
        &EncodingKey::from_secret(ENV.jwt_refresh_secret.as_ref()),
    )?;

    info!("Refresh token generated successfully for user: {}", user_id);

    Ok(refresh_token)
}
