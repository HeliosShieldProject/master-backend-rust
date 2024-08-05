use crate::{
    dto::{auth::internal::AccessToken, device::response::Device, response::success::Response},
    enums::errors::external::ExternalError,
    services::device_service,
};
use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use tracing::info;

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
    State(pool): State<Pool>,
) -> Result<Response<Vec<Device>>, ExternalError> {
    let devices: Vec<Device> = device_service::get_devices(&pool, &claims.user_id)
        .await?
        .into_iter()
        .map(Device::from)
        .collect();

    info!("Devices retrieved successfully: {}", devices.len());

    Ok(Response::new(StatusCode::OK, "Devices retrieved successfully").with_data(devices))
}
