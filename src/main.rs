use crate::logger::info;
use config::ENV;
use deadpool_diesel::postgres::{Manager, Pool};
use routers::app_router;
use swagger::ApiDoc;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod data;
mod dto;
mod enums;
mod guards;
mod handlers;
mod logger;
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
    let manager = Manager::new(&ENV.database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();
    let state = AppState { pool };
    let app = match ENV.rust_env.as_str() {
        "developmentA" => app_router(state.clone())
            .with_state(state)
            .merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi())),
        _ => app_router(state.clone()).with_state(state),
    };

    let listener = TcpListener::bind(format!("localhost:{}", ENV.master_backend_port))
        .await
        .unwrap();

    info(
        format!("Listening on {}", listener.local_addr().unwrap()).as_str(),
        "main".to_string(),
    );
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
