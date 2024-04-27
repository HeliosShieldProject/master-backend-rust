use crate::{
    data::repositories::device_repository,
    dto::{auth::AccessToken, device},
    enums::errors::response::{to_response, ResponseError},
    AppState,
};
use axum::{extract::State, Json};

pub async fn get_devices(
    claims: AccessToken,
    State(state): State<AppState>,
) -> Result<Json<Vec<device::Device>>, ResponseError> {
    let devices = device_repository::get_devices(&state.pool, &claims.user_id)
        .await
        .map_err(to_response)?;
    Ok(Json(devices))
}
