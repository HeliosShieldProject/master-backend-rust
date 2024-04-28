use crate::{
    dto::{
        auth::internal::AccessToken, device::response::Device, response::success::SuccessResponse,
    },
    enums::errors::response::{to_response, ResponseError},
    services::device_service,
    AppState,
};
use axum::{extract::State, http::StatusCode};

pub async fn get_devices(
    claims: AccessToken,
    State(state): State<AppState>,
) -> Result<SuccessResponse<Vec<Device>>, ResponseError> {
    let devices = device_service::get_devices(&state.pool, &claims.user_id)
        .await
        .map_err(to_response)?
        .into_iter()
        .map(|device| Device::from(device))
        .collect();

    Ok(SuccessResponse::new(StatusCode::OK, "Devices retrieved successfully").with_data(devices))
}
