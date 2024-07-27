use axum::{routing::get, Router};
use config::ENV;
use deadpool_diesel::postgres::{Manager, Pool};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use routers::app_router;
use std::future::ready;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// In order to fix SSL error while building for the x86_64-unknown-linux-musl target
extern crate openssl;
#[allow(unused_imports)]
#[macro_use]
extern crate diesel;

mod config;
mod data;
mod dto;
mod enums;
mod guards;
mod handlers;
mod routers;
mod services;
// mod swagger;
mod tests;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
}

async fn start_main_server() {
    let manager = Manager::new(&ENV.database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();
    let state = AppState { pool };
    let app = app_router(state.clone()).with_state(state);

    let listener = TcpListener::bind(&ENV.master_backend_url).await.unwrap();
    info!(
        "Main server listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn metrics_app() -> Router {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    let recorder_handle = PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap();

    Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}

async fn start_metrics_server() {
    let app = metrics_app();

    let listener = TcpListener::bind(&ENV.master_metrics_url).await.unwrap();
    info!(
        "Metrcis server listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (_main_server, _metrics_server) = tokio::join!(start_main_server(), start_metrics_server());
}
