use crate::{
    handlers::session::{close_session, create_session},
    AppState,
};
use axum::{
    routing::{post, put},
    Router,
};

pub fn session_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_session))
        .route("/", put(close_session))
        .with_state(state)
}
