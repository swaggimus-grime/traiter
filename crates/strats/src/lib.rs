mod buy_and_hold;
mod sma_cross;

use core::{Order, ExecutionReport, OrderSide};
use data::MarketBar;

pub trait Strategy {
    fn on_market_event(&mut self, event: &MarketBar) -> Vec<Order>;
    fn on_fill(&mut self, report: &ExecutionReport);
}

