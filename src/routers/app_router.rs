use {
    super::{auth_router, device_router, session_router},
    crate::{middleware::logging_middleware, AppState},
    axum::{http::StatusCode, middleware, response::IntoResponse, routing::get, Router},
};

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth_router(state.clone()))
        .nest("/session", session_router(state.clone()))
        .nest("/device", device_router(state.clone()))
        .fallback(handler_404)
        .layer(middleware::from_fn(logging_middleware))
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found.",
    )
}
