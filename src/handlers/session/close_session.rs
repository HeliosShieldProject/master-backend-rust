use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use tracing::info;

use crate::{
    dto::{auth::internal::AccessToken, response::success::Response},
    enums::errors::external::Result,
    services::session,
};

pub async fn close_session(
    claims: AccessToken,
    State(pool): State<Pool>,
) -> Result<Response<String>> {
    let session_id = session::close(&pool, &claims.device_id).await?;

    info!("Closed session successfully: {}", session_id);

    Ok(Response::new(StatusCode::OK, "Closed session successfully"))
}
