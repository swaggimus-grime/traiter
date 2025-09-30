mod live;

use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::routes::live::stream_stock;
use crate::state::BackendState;

pub fn api_routes() -> Router<Arc<BackendState>> {
    Router::new()
        .route("/live/stock", get(stream_stock))
        .route("/strategies", get(get_strategies).post(create_strategy))
        .route("/strategies/:id", delete(delete_strategy))
        .route("/strategies/:id/backtest", post(run_backtest))
        .route("/strategies/:id/backtest/:backtest_id", get(get_backtest_results))
}