use crate::{
    dto::auth::{internal::AccessToken, request::ChangePasswordRequest},
    enums::errors::response::{to_response, AuthError, ResponseError},
    services::user_service,
    utils::hash,
    AppState,
};
use axum::{extract::State, Json};

pub async fn change_password(
    claims: AccessToken,
    State(state): State<AppState>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<String, ResponseError> {
    let user = user_service::get_by_id(&state.pool, &claims.user_id)
        .await
        .map_err(to_response)?;

    if hash::verify_password(&payload.password, &user.password).is_ok() {
        return Err(ResponseError::AuthError(AuthError::PasswordIsSame));
    }
    let hashed_password = hash::hash_password(&payload.password).map_err(to_response)?;

    user_service::change_password(&state.pool, &claims.user_id, &hashed_password)
        .await
        .map_err(to_response)?;
    Ok("Password changed".to_string())
}
