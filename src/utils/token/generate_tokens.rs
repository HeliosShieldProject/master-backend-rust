use super::{generate_access_token, generate_refresh_token};
use crate::enums::errors::internal::InternalError;
use tokio::try_join;

pub async fn generate_tokens(
    user_id: &str,
    device_id: &str,
) -> Result<(String, String), InternalError> {
    let (access_token, refresh_token) = try_join!(
        generate_access_token(user_id, device_id),
        generate_refresh_token(user_id, device_id)
    )?;

    Ok((access_token, refresh_token))
}
