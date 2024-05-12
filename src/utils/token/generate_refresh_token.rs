use crate::config::ENV;
use crate::dto::auth::internal::RefreshToken;
use crate::enums::errors::internal::{to_internal, InternalError};
use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use tracing::{error, info};
use uuid::Uuid;

pub async fn generate_refresh_token(
    user_id: &str,
    device_id: &str,
) -> Result<String, InternalError> {
    let now = Local::now();
    let refresh_token_values = RefreshToken {
        exp: (now + Duration::days(7)).timestamp(),
        iat: now.timestamp(),
        user_id: Uuid::parse_str(&user_id)
            .map_err(to_internal)
            .map_err(|e| {
                error!("{}", e);
                e
            })?,
        device_id: Uuid::parse_str(&device_id)
            .map_err(to_internal)
            .map_err(|e| {
                error!("{}", e);
                e
            })?,
    };

    encode(
        &Header::default(),
        &refresh_token_values,
        &EncodingKey::from_secret(ENV.jwt_refresh_secret.as_ref()),
    )
    .map_err(to_internal)
    .map_err(|e| {
        error!("{}", e);
        e
    })
    .map(|refresh_token| {
        info!("Refresh token generated for user: {}", user_id);
        refresh_token
    })
}
