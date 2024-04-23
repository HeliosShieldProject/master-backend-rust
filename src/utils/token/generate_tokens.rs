use crate::enums::AuthError;

use super::{generate_access_token, generate_refresh_token};

pub async fn generate_tokens(
    user_id: &str,
    device_id: &str,
) -> Result<(String, String), AuthError> {
    let access_token = generate_access_token(user_id, device_id)
        .await
        .map_err(|_| AuthError::TokenCreation)?;
    let refresh_token = generate_refresh_token(user_id, device_id)
        .await
        .map_err(|_| AuthError::TokenCreation)?;

    Ok((access_token, refresh_token))
}
