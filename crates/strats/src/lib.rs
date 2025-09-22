mod buy_and_hold;
mod sma_cross;

use serde::{Deserialize, Serialize};
use dnn_core::market::Candle;
use dnn_core::Order;
use dnn_core::time::Timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub order_id: u64,
    pub filled_qty: f64,
    pub fill_price: f64,
    pub ts: Timestamp,
}

pub trait Strategy {
    fn on_market_event(&mut self, event: &Candle) -> Vec<Order>;
    fn on_fill(&mut self, report: &ExecutionReport);
}

