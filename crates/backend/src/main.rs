mod routes;
mod config;

use std::net::SocketAddr;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::info;
use crate::config::BackendConfig;
use crate::routes::api_routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = BackendConfig::load().expect("Failed to load config");

    let app = Router::new()
        .merge(api_routes())
        .with_state(config.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("🚀 Listening on http://{}", addr);

    // Start the server
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
