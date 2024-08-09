use crate::{
    handlers::session::{close_session, create_session, get_history},
    AppState,
};
use axum::{
    routing::{get, post, put},
    Router,
};

pub fn session_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_session))
        .route("/", put(close_session))
        .route("/history", get(get_history))
        .with_state(state)
}
