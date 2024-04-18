use config::{config, Config};
use deadpool_diesel::postgres::{Manager, Pool};
use listenfd::ListenFd;
use routers::app_router;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod controllers;
mod routers;
mod data;

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
    env: Config,
}

#[tokio::main]
async fn main() {
    let config = config().await;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let manager = Manager::new(config.database_url(), deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();
    let state = AppState {
        pool,
        env: config.clone(),
    };
    let app = app_router(state.clone()).with_state(state);

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
