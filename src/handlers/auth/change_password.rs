use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use tracing::info;

use crate::{
    dto::{
        auth::{internal::AccessToken, request::ChangePasswordRequest},
        response::success::Response,
    },
    enums::errors::external::Result,
    extractors::Json,
    services::user,
};

pub async fn change_password(
    claims: AccessToken,
    State(pool): State<Pool>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Response<String>> {
    user::change_password(&pool, &claims.user_id, &payload.password).await?;

    info!("Password changed successfully for user: {}", claims.user_id);

    Ok(Response::new(
        StatusCode::OK,
        "Password changed successfully",
    ))
}
