use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use tracing::info;

use crate::{
    dto::{auth::internal::AccessToken, device::response::Device, response::success::Response},
    enums::errors::external::Result,
    services::device,
};

pub async fn get_devices(
    claims: AccessToken,
    State(pool): State<Pool>,
) -> Result<Response<Vec<Device>>> {
    let devices: Vec<Device> = device::get_many(&pool, &claims.user_id)
        .await?
        .into_iter()
        .map(Device::from)
        .collect();

    info!("Devices retrieved successfully: {}", devices.len());

    Ok(Response::new(StatusCode::OK, "Devices retrieved successfully").with_data(devices))
}
