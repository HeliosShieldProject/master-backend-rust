use crate::{
    data::repositories::user_repository,
    dto::auth::internal::AccessToken,
    enums::errors::response::{to_response, AuthError, ResponseError},
    utils::hash,
    AppState,
};
use axum::{extract::State, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    password: String,
}

pub async fn change_password(
    claims: AccessToken,
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<String, ResponseError> {
    let user = user_repository::get_by_id(&state.pool, &claims.user_id)
        .await
        .map_err(to_response)?;

    if hash::verify_password(&payload.password, &user.password).is_ok() {
        return Err(ResponseError::AuthError(AuthError::PasswordIsSame));
    }
    let hashed_password = hash::hash_password(&payload.password).map_err(to_response)?;

    user_repository::change_password(&state.pool, &claims.user_id, &hashed_password)
        .await
        .map_err(to_response)?;
    Ok("Password changed".to_string())
}
