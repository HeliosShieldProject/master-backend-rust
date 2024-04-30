use crate::{
    dto::{
        auth::internal::AccessToken, device::response::Device, response::success::SuccessResponse,
    },
    enums::errors::response::{to_response, ResponseError},
    services::device_service,
    AppState,
};
use axum::{extract::State, http::StatusCode};

#[utoipa::path(
    tag = "Device",
    post,
    path = "/device",
    security(
        ("access_token" = ["Bearer"])
    ),
    responses(
        (
            status = 200,
            description = "Devices retrieved successfully",
            body = Vec<Device>,
            example = json!({
                "message": "Devices retrieved successfully",
                "data": [
                    {
                        "id": "some-uuid",
                        "name": "some-name",
                        "os": "Android",
                        "status": "LoggedIn",
                    },
                    {
                        "id": "some-uuid",
                        "name": "some-name",
                        "os": "Windows",
                        "status": "LoggedOut",
                    }
                ]
            })
        ),
        (
            status = 401,
            description = "Wrong token",
            body = (),
            example = json!({
                "message": "Wrong token",
                "error": "WrongToken"
            })
        )
    )
)]
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
