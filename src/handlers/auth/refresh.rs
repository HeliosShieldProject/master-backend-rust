use crate::{
    dto::auth::{RefreshToken, Response},
    enums::AuthError,
    utils::token,
};
use axum::Json;

pub async fn refresh(claims: RefreshToken) -> Result<Json<Response>, AuthError> {
    let access_token =
        token::generate_access_token(&claims.user_id.to_string(), &claims.device_id.to_string())
            .await
            .map_err(|_| AuthError::TokenCreation)?;

    let refresh_token =
        token::generate_refresh_token(&claims.user_id.to_string(), &claims.device_id.to_string())
            .await
            .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(Response::new(access_token, refresh_token)))
}
