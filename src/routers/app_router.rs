use {
    super::{auth_router, device_router, session_router},
    crate::{
        logger::{info_request, info_response, types::{RequestLog, ResponseLog}},
        AppState,
    },
    axum::{
        body::Body,
        http::{header, Request, StatusCode},
        response::{IntoResponse, Response},
        routing::get,
        Router,
    },
    std::{sync::Arc, time::Duration},
    tower::ServiceBuilder,
    tower_http::{trace::TraceLayer, ServiceBuilderExt},
    tracing::Span,
};

pub fn app_router(state: AppState) -> Router<AppState> {
    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION].into();
    let middleware = ServiceBuilder::new()
        .sensitive_request_headers(sensitive_headers.clone())
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<Body>, _: &Span| {
                    info_request(RequestLog {
                        method: request.method().to_string(),
                        url: request.uri().to_string(),
                        headers: request.headers().clone(),
                    })
                })
                .on_response(|response: &Response, latency: Duration, _: &Span| {
                    info_response(ResponseLog {
                        status: response.status().as_u16(),
                        headers: response.headers().clone(),
                    })
                }),
        );

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth_router(state.clone()))
        .nest("/session", session_router(state.clone()))
        .nest("/device", device_router(state.clone()))
        .fallback(handler_404)
        .layer(middleware)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found.",
    )
}
