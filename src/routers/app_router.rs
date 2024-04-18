use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use crate::AppState;

use super::auth_router::auth_router;

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth_router(state.clone()))
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found.",
    )
}
