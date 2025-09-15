mod live;

use axum::Router;
use axum::routing::get;
use crate::config::BackendConfig;
use crate::routes::live::stream_stock;

pub fn api_routes() -> Router<BackendConfig> {
    Router::new()
        .route("/live/stock", get(stream_stock))
}