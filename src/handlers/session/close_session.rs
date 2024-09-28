use axum::{extract::State, http::StatusCode};
use tracing::info;

use crate::{
    dto::{auth::internal::AccessToken, response::success::Response},
    enums::errors::external::Result,
    services::session,
    state::AppState,
};

pub async fn close_session(
    claims: AccessToken,
    State(state): State<AppState>,
) -> Result<Response<String>> {
    let session_id = session::close(&state.pool, &state.agent_state, &claims.device_id).await?;

    info!("Closed session successfully: {}", session_id);

    Ok(Response::new(StatusCode::OK, "Closed session successfully"))
}
