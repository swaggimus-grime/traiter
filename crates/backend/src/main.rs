mod routes;
mod config;
mod services;
mod state;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing::info;
use crate::config::BackendConfig;
use crate::routes::api_routes;
use crate::state::BackendState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(BackendState::new().unwrap());
    let app = Router::new()
        .merge(api_routes())
        .with_state(state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], state.config.port));
    info!("🚀 Listening on http://{}", addr);

    // Start the server
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
