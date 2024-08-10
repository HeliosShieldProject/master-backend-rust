use axum::http::StatusCode;
use tracing::info;

use crate::{
    dto::{
        auth::{internal::RefreshToken, response::Tokens},
        response::success::Response,
    },
    enums::errors::external::Result,
    utils::token::generate_tokens,
};

pub async fn refresh(claims: RefreshToken) -> Result<Response<Tokens>> {
    let tokens =
        generate_tokens(&claims.user_id.to_string(), &claims.device_id.to_string()).await?;

    info!("Tokens refreshed for user: {:?}", claims.user_id);

    Ok(Response::new(StatusCode::OK, "Tokens refreshed successfully").with_data(tokens))
}
