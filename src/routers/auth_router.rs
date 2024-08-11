use axum::{
    routing::{get, post, put},
    Router,
};

use crate::{
    handlers::auth::{
        authorize, change_password, confirm_email, logout, refresh, sign_in, sign_up,
    },
    AppState,
};

pub fn auth_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/sign-in", post(sign_in))
        .route("/sign-up", post(sign_up))
        .route("/authorize", post(authorize))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/change-password", put(change_password))
        .route("/confirm-email", get(confirm_email))
        .with_state(state)
}
