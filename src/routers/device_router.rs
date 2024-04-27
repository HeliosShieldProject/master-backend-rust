use crate::{handlers::device::get_devices, AppState};
use axum::{routing::get, Router};

pub fn device_router(state: AppState) -> Router<AppState> {
    Router::new().route("/", get(get_devices)).with_state(state)
}
