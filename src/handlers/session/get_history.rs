use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use tracing::info;

use crate::{
    dto::{
        auth::internal::AccessToken,
        response::success::Response,
        session::{internal::SessionHistory, request::Params},
    },
    enums::errors::external::ExternalError,
    extractors::Json,
    services::session_service,
};

pub async fn get_history(
    claims: AccessToken,
    State(pool): State<Pool>,
    Json(payload): Json<Params>,
) -> Result<Response<Vec<SessionHistory>>, ExternalError> {
    info!("Getting session history");

    let history = session_service::get_history(&pool, &claims.user_id, &payload).await?;

    Ok(Response::new(StatusCode::OK, "Successfully got session history").with_data(history))
}
