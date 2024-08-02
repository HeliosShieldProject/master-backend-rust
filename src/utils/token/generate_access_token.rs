use crate::config::ENV;
use crate::dto::auth::internal::AccessToken;
use crate::enums::errors::internal::InternalError;
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use tracing::info;
use uuid::Uuid;

pub async fn generate_access_token(
    user_id: &str,
    device_id: &str,
) -> Result<String, InternalError> {
    let now = Local::now();
    let access_token_values = AccessToken {
        exp: (now + Duration::hours(1)).timestamp(),
        iat: now.timestamp(),
        user_id: Uuid::parse_str(&user_id)?,
        device_id: Uuid::parse_str(&device_id)?,
    };

    let access_token = encode(
        &Header::default(),
        &access_token_values,
        &EncodingKey::from_secret(ENV.jwt_access_secret.as_ref()),
    )?;

    info!("Access token generated successfully for user: {}", user_id);

    Ok(access_token)
}
