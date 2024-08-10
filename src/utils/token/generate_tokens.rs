use tokio::try_join;
use tracing::info;

use super::{generate_access_token, generate_refresh_token};
use crate::{dto::auth::response::Tokens, enums::errors::internal::Result};

pub async fn generate_tokens(user_id: &str, device_id: &str) -> Result<Tokens> {
    let (access_token, refresh_token) = try_join!(
        generate_access_token(user_id, device_id),
        generate_refresh_token(user_id, device_id)
    )?;

    info!("Tokens generated for user: {}", user_id);
    Ok(Tokens {
        access_token,
        refresh_token,
    })
}
