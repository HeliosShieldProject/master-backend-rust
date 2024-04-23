use crate::{
    dto::auth::{RefreshToken, Response},
    enums::AuthError,
    utils::token::generate_tokens,
};
use axum::Json;

pub async fn refresh(claims: RefreshToken) -> Result<Json<Response>, AuthError> {
    let (access_token, refresh_token) =
        generate_tokens(&claims.user_id.to_string(), &claims.device_id.to_string())
            .await
            .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(Response::new(access_token, refresh_token)))
}
