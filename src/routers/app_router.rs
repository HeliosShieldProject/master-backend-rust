use {
    super::{auth_router, device_router, session_router},
    crate::AppState,
    axum::{
        extract::MatchedPath,
        http::{Request, StatusCode},
        response::IntoResponse,
        routing::get,
        Router,
    },
    tower_http::trace::TraceLayer,
    tracing::info_span,
};

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth_router(state.clone()))
        .nest("/session", session_router(state.clone()))
        .nest("/device", device_router(state.clone()))
        .fallback(handler_404)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http",
                    method = ?request.method(),
                    path,
                )
            }),
        )
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found.",
    )
}
