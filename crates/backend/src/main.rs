mod routes;
mod config;

use std::net::SocketAddr;
use axum::{routing::get, Router};
use dotenv::dotenv;
use tokio::net::TcpListener;
use tracing::info;
use crate::config::Config;
use crate::routes::api_routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();

    let app = Router::new()
        .merge(api_routes())
        .with_state(config);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("🚀 Listening on http://{}", addr);

    // Start the server
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
