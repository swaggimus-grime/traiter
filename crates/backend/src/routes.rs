mod live;

use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::routes::live::stream_stock;
use crate::state::BackendState;

pub fn api_routes() -> Router<Arc<BackendState>> {
    Router::new()
        .route("/live/stock", get(stream_stock))
}