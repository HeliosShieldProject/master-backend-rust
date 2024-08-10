use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use tracing::info;

use crate::{
    dto::{
        auth::internal::AccessToken,
        response::success::Response,
        session::{internal::SessionHistory, request::Params},
    },
    enums::errors::external::Result,
    extractors::Json,
    services::session,
};

pub async fn get_history(
    claims: AccessToken,
    State(pool): State<Pool>,
    Json(payload): Json<Params>,
) -> Result<Response<Vec<SessionHistory>>> {
    info!("Getting session history");

    let history = session::get_history(&pool, &claims.user_id, &payload).await?;

    Ok(Response::new(StatusCode::OK, "Successfully got session history").with_data(history))
}
