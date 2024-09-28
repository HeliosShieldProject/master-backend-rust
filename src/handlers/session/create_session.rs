use axum::{extract::State, http::StatusCode};
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
    state::AppState,
};

pub async fn create_session(
    claims: AccessToken,
    State(state): State<AppState>,
    Json(payload): Json<CreateSession>,
) -> Result<Response<Session>> {
    let session = session::create(
        &state.pool,
        &state.agent_state,
        &payload.country,
        &payload.protocol,
        &claims.device_id,
    )
    .await?;

    info!("Session created successfully: {}", session.session_id);

    Ok(Response::new(StatusCode::CREATED, "Session created successfully").with_data(session))
}
