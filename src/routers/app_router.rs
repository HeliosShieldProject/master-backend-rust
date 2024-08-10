use std::time::Instant;

use axum::{
    extract,
    extract::MatchedPath,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use super::{auth_router, device_router, session_router};
use crate::AppState;

async fn track_metrics(req: extract::Request, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::counter!("http_requests_total", &labels).increment(1);
    metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);

    response
}

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
        .route_layer(middleware::from_fn(track_metrics))
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found.",
    )
}
