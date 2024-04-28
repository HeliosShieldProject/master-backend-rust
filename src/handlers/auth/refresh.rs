use crate::{
    dto::{
        auth::{internal::RefreshToken, response::Tokens},
        response::success::SuccessResponse,
    },
    enums::errors::response::{to_response, ResponseError},
    utils::token::generate_tokens,
};
use axum::http::StatusCode;

pub async fn refresh(claims: RefreshToken) -> Result<SuccessResponse<Tokens>, ResponseError> {
    let tokens = generate_tokens(&claims.user_id.to_string(), &claims.device_id.to_string())
        .await
        .map_err(to_response)?;

    Ok(SuccessResponse::new(StatusCode::OK, "Tokens refreshed successfully").with_data(tokens))
}
