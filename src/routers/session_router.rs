use axum::{routing::post, Router};
use crate::{handlers::session::create_session, AppState};

pub fn session_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_session))
        .with_state(state)
}
