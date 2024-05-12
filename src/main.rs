use config::ENV;
use deadpool_diesel::postgres::{Manager, Pool};
use routers::app_router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        .with(tracing_subscriber::fmt::layer())
        .init();

    let manager = Manager::new(&ENV.database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();
    let state = AppState { pool };
    let app = app_router(state.clone()).with_state(state);

    let listener = TcpListener::bind(&ENV.master_backend_url).await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
