use crate::{
    data::repositories::device_repository, dto::auth::AccessToken, enums::AuthError, AppState,
};
use axum::extract::State;

pub async fn logout(
    State(state): State<AppState>,
    access_token: AccessToken,
) -> Result<String, AuthError> {
    let _ = device_repository::logout_device(&state.pool, &access_token.device_id).await;
    Ok("Logged out".to_string())
}
