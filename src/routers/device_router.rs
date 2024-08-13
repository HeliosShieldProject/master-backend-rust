use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::device::{get_devices, revoke_device},
    AppState,
};

pub fn device_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(get_devices))
        .with_state(state)
        .route("/revoke/:device_id", post(revoke_device))
}
