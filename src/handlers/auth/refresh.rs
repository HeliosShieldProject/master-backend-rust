use crate::{
    dto::auth::{internal::RefreshToken, response::Tokens},
    enums::errors::response::{to_response, ResponseError},
    utils::token::generate_tokens,
};
use axum::Json;

pub async fn refresh(claims: RefreshToken) -> Result<Json<Tokens>, ResponseError> {
    let (access_token, refresh_token) =
        generate_tokens(&claims.user_id.to_string(), &claims.device_id.to_string())
            .await
            .map_err(to_response)?;

    Ok(Json(Tokens {
        access_token,
        refresh_token,
    }))
}
