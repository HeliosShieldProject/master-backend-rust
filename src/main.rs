use config::ENV;
use deadpool_diesel::postgres::{Manager, Pool};
use routers::app_router;
use swagger::ApiDoc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod data;
mod dto;
mod enums;
mod guards;
mod handlers;
mod routers;
mod services;
mod swagger;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let manager = Manager::new(&ENV.database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();
    let state = AppState { pool };
    let app = match ENV.rust_env.as_str() {
        "development" => app_router(state.clone())
            .with_state(state)
            .merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi())),
        _ => app_router(state.clone()).with_state(state),
    };

    let listener = TcpListener::bind(format!("localhost:{}", ENV.master_backend_port))
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
