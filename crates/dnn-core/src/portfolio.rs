use std::collections::HashMap;
use crate::{OrderSide, Position};

#[derive(Debug)]
pub struct Portfolio {
    pub cash: f64,
    pub positions: HashMap<String, Position>,
}

impl Portfolio {
    pub fn new(starting_cash: f64) -> Self {
        Self {
            cash: starting_cash,
            positions: HashMap::new(),
        }
    }

    pub fn apply_fill(&mut self, symbol: &str, side: OrderSide, qty: f64, price: f64) {
        let entry = self.positions
            .entry(symbol.to_string())
            .or_insert_with(|| Position::new(symbol.to_string()));

        // cash movement
        match side {
            OrderSide::Buy => {
                self.cash -= qty * price;
            }
            OrderSide::Sell => {
                self.cash += qty * price;
            }
        }

        // update position
        entry.update(side, qty, price);
    }

    pub fn total_value(&self, prices: &HashMap<String, f64>) -> f64 {
        let mut value = self.cash;
        for (sym, pos) in &self.positions {
            if let Some(price) = prices.get(sym) {
                value += pos.market_value(*price);
            }
        }
        value
    }
}
