use crate::{ExecutionReport, Strategy};

pub struct BuyAndHold;

impl Strategy for BuyAndHold {
    fn on_market_event(&mut self, event: &core::market::Candle) -> Vec<core::Order> {
        todo!()
    }

    fn on_fill(&mut self, report: &ExecutionReport) {
        todo!()
    }
}