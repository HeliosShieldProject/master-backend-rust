use axum::{routing::post, Router};

use crate::{
    handlers::auth::{logout, refresh, sign_in, sign_up},
    AppState,
};

pub fn auth_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/sign-in", post(sign_in))
        .route("/sign-up", post(sign_up))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/change-password", post(|| async { "Change password" }))
        .with_state(state)
}
