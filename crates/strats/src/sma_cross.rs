use data::MarketBar;
use crate::Strategy;

pub struct SmaCross {
    pub short: usize,
    pub long: usize,
    prices: Vec<f64>,
}

impl Strategy for SmaCross {
    fn on_market_event(&mut self, bar: &MarketBar) -> Vec<Order> {
        self.prices.push(bar.close);
        if self.prices.len() < self.long {
            return vec![];
        }

        let short_avg = self.prices[self.prices.len()-self.short..].iter().copied().sum::<f64>() / self.short as f64;
        let long_avg  = self.prices[self.prices.len()-self.long..].iter().copied().sum::<f64>() / self.long as f64;

        if short_avg > long_avg {
            vec![Order {
                id: bar.ts.timestamp() as u64,
                symbol: "AAPL".into(),
                qty: 10.0,
                price: None,
                side: OrderSide::Buy,
            }]
        } else {
            vec![]
        }
    }

    fn on_fill(&mut self, report: &ExecutionReport) {
        println!("Order filled at {} for qty {}", report.fill_price, report.filled_qty);
    }
}
