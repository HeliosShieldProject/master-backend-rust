use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use deadpool_diesel::postgres::Pool;
use tracing::info;
use uuid::Uuid;

use crate::{
    dto::{auth::internal::AccessToken, response::success::Response},
    enums::errors::external::Result,
    services::device,
};

pub async fn revoke_device(
    claims: AccessToken,
    State(pool): State<Pool>,
    Path(device_id): Path<Uuid>,
) -> Result<Response<()>> {
    device::revoke(&pool, claims, &device_id).await?;

    info!("Device revoked successfully: {:?}", device_id);

    Ok(Response::new(StatusCode::OK, "Device revoked successfully"))
}
