use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tracing::info;
use uuid::Uuid;

use crate::{
    dto::{auth::internal::AccessToken, response::success::Response},
    enums::errors::external::Result,
    services::device,
    state::AppState,
};

pub async fn revoke_device(
    claims: AccessToken,
    State(state): State<AppState>,
    Path(device_id): Path<Uuid>,
) -> Result<Response<()>> {
    device::revoke(&state.pool, &state.agent_state, claims, &device_id).await?;

    info!("Device revoked successfully: {:?}", device_id);

    Ok(Response::new(StatusCode::OK, "Device revoked successfully"))
}
