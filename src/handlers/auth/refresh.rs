use crate::{
    dto::{
        auth::{internal::RefreshToken, response::Tokens},
        response::success::Response,
    },
    enums::errors::response::{to_response, ResponseError},
    utils::token::generate_tokens,
};
use axum::http::StatusCode;
use tracing::{error, info};

#[utoipa::path(
    tag = "Auth",
    post,
    path = "/auth/refresh",
    security(
        ("refresh_token" = ["Bearer"])
    ),
    responses(
        (
            status = 200,
            description = "Tokens refreshed successfully",
            body = Tokens,
            example = json!({
                "message": "Tokens refreshed successfully",
                "data": {
                    "access_token": "access",
                    "refresh_token": "refresh"
                }
            })
        ),
        (
            status = 401,
            description = "Wrong token",
            body = (),
            example = json!({
                "message": "Wrong token",
                "error": "WrongToken"
            })
        ),
    )
)]
pub async fn refresh(claims: RefreshToken) -> Result<Response<Tokens>, ResponseError> {
    let tokens = generate_tokens(&claims.user_id.to_string(), &claims.device_id.to_string())
        .await
        .map_err(|e| {
            error!("Failed to generate tokens: {}", e);
            e
        })
        .map_err(to_response)?;

    info!("Tokens refreshed for user: {:?}", claims.user_id);
    Ok(Response::new(StatusCode::OK, "Tokens refreshed successfully").with_data(tokens))
}
