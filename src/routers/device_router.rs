use axum::{routing::get, Router};

use crate::{handlers::device::get_devices, AppState};

pub fn device_router(state: AppState) -> Router<AppState> {
    Router::new().route("/", get(get_devices)).with_state(state)
}
