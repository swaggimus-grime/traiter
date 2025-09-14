pub mod market;
mod tests;
pub mod portfolio;
pub mod time;
mod error;

use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrderSide { Buy, Sell }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub qty: f64,
    pub price: Option<f64>, // None = market
    pub side: OrderSide,
}

/// Trading signal types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Signal::Buy => write!(f, "BUY"),
            Signal::Sell => write!(f, "SELL"),
            Signal::Hold => write!(f, "HOLD"),
        }
    }
}

/// Order types for trading
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit { price: f64 },
    Stop { price: f64 },
    StopLimit { stop_price: f64, limit_price: f64 },
}

#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub qty: f64,
    pub avg_price: f64,
    pub realized_pnl: f64,
}

impl Position {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            qty: 0.0,
            avg_price: 0.0,
            realized_pnl: 0.0,
        }
    }

    pub fn update(&mut self, side: OrderSide, qty: f64, price: f64) {
        match side {
            OrderSide::Buy => {
                let new_qty = self.qty + qty;
                self.avg_price = (self.avg_price * self.qty + price * qty) / new_qty;
                self.qty = new_qty;
            }
            OrderSide::Sell => {
                if qty <= self.qty {
                    // closing long
                    let pnl = (price - self.avg_price) * qty;
                    self.realized_pnl += pnl;
                    self.qty -= qty;
                } else {
                    // shorting more than current long (flip)
                    let pnl = (price - self.avg_price) * self.qty;
                    self.realized_pnl += pnl;
                    self.qty = -(qty - self.qty);
                    self.avg_price = price;
                }
            }
        }
    }

    pub fn market_value(&self, price: f64) -> f64 {
        self.qty * price
    }

    pub fn unrealized_pnl(&self, price: f64) -> f64 {
        (price - self.avg_price) * self.qty
    }
}

