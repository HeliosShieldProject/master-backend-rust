use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use tracing::info;

use crate::{
    dto::{
        auth::internal::AccessToken,
        response::success::Response,
        session::{request::CreateSession, response::Session},
    },
    enums::errors::external::Result,
    extractors::Json,
    services::session,
};

pub async fn create_session(
    claims: AccessToken,
    State(pool): State<Pool>,
    Json(payload): Json<CreateSession>,
) -> Result<Response<Session>> {
    let session = session::create(&pool, &claims.device_id, &payload.country).await?;

    info!("Session created successfully: {}", session.session_id);

    Ok(Response::new(StatusCode::CREATED, "Session created successfully").with_data(session))
}
