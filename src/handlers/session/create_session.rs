use crate::{
    data::enums::Country,
    dto::{
        auth::internal::AccessToken,
        response::success::SuccessResponse,
        session::{request, response},
    },
    enums::errors::response::{to_response, ResponseError},
    services::session_service,
    AppState,
};
use axum::{extract::State, http::StatusCode, Json};

pub async fn create_session(
    claims: AccessToken,
    State(state): State<AppState>,
    Json(payload): Json<request::CreateSession>,
) -> Result<SuccessResponse<response::Session>, ResponseError> {
    let country = Country::from_str(&payload.country).map_err(to_response)?;
    let session = session_service::create_session(&state.pool, &claims.device_id, &country)
        .await
        .map_err(to_response)?;

    Ok(
        SuccessResponse::new(StatusCode::CREATED, "Session created successfully")
            .with_data(response::Session::from(session)),
    )
}
