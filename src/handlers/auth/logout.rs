use crate::{
    data::repositories::device_repository,
    dto::auth::AccessToken,
    enums::errors::response::{to_response, ResponseError},
    AppState,
};
use axum::extract::State;

pub async fn logout(
    State(state): State<AppState>,
    access_token: AccessToken,
) -> Result<String, ResponseError> {
    let _ = device_repository::logout_device(&state.pool, &access_token.device_id)
        .await
        .map_err(to_response);
    Ok("Logged out".to_string())
}
