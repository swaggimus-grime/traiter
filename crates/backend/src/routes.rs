mod live;

use axum::Router;
use axum::routing::get;
use crate::routes::live::stream_stock;

pub fn api_routes() -> Router {
    Router::<()>::new()
        .without_v07_checks()
        .route("/live/stock", get(stream_stock))
}