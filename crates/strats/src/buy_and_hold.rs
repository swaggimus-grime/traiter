use dnn_core::market::Candle;
use dnn_core::Order;
use crate::{ExecutionReport, Strategy};

pub struct BuyAndHold;

impl Strategy for BuyAndHold {
    fn on_market_event(&mut self, event: &Candle) -> Vec<Order> {
        todo!()
    }

    fn on_fill(&mut self, report: &ExecutionReport) {
        todo!()
    }
}