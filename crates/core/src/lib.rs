mod ohlcv;
mod tests;

use std::collections::HashMap;
use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub use crate::ohlcv::OHLCV;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub order_id: u64,
    pub filled_qty: f64,
    pub fill_price: f64,
    pub ts: DateTime<Utc>,
}

/// Simple trait for strategie

/// Represents different timeframes for market data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Timeframe {
    Minute1,
    Minute5,
    Minute15,
    Minute30,
    Hour1,
    Hour4,
    Day1,
    Week1,
    Month1,
}

impl Timeframe {
    /// Convert timeframe to seconds
    pub fn to_seconds(&self) -> i64 {
        match self {
            Timeframe::Minute1 => 60,
            Timeframe::Minute5 => 300,
            Timeframe::Minute15 => 900,
            Timeframe::Minute30 => 1800,
            Timeframe::Hour1 => 3600,
            Timeframe::Hour4 => 14400,
            Timeframe::Day1 => 86400,
            Timeframe::Week1 => 604800,
            Timeframe::Month1 => 2592000, // Approximate
        }
    }
}

impl fmt::Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Timeframe::Minute1 => "1m",
            Timeframe::Minute5 => "5m",
            Timeframe::Minute15 => "15m",
            Timeframe::Minute30 => "30m",
            Timeframe::Hour1 => "1h",
            Timeframe::Hour4 => "4h",
            Timeframe::Day1 => "1d",
            Timeframe::Week1 => "1w",
            Timeframe::Month1 => "1M",
        };
        write!(f, "{}", s)
    }
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

/// Market data series - a collection of OHLCV data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub timeframe: Timeframe,
    pub data: Vec<OHLCV>,
}

impl MarketData {
    pub fn new(symbol: String, timeframe: Timeframe) -> Self {
        MarketData {
            symbol,
            timeframe,
            data: Vec::new(),
        }
    }

    /// Add new OHLCV data point
    pub fn add(&mut self, ohlcv: OHLCV) {
        self.data.push(ohlcv);
        // Keep data sorted by timestamp
        self.data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }

    /// Get the latest OHLCV data
    pub fn latest(&self) -> Option<&OHLCV> {
        self.data.last()
    }

    /// Get OHLCV data at specific index from the end
    pub fn get_from_end(&self, index: usize) -> Option<&OHLCV> {
        if index < self.data.len() {
            Some(&self.data[self.data.len() - 1 - index])
        } else {
            None
        }
    }

    /// Get closing prices as a vector
    pub fn closes(&self) -> Vec<f64> {
        self.data.iter().map(|ohlcv| ohlcv.close).collect()
    }

    /// Get high prices as a vector
    pub fn highs(&self) -> Vec<f64> {
        self.data.iter().map(|ohlcv| ohlcv.high).collect()
    }

    /// Get low prices as a vector
    pub fn lows(&self) -> Vec<f64> {
        self.data.iter().map(|ohlcv| ohlcv.low).collect()
    }

    /// Get volumes as a vector
    pub fn volumes(&self) -> Vec<f64> {
        self.data.iter().map(|ohlcv| ohlcv.volume).collect()
    }

    /// Get data length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if data is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Custom error types for price data
#[derive(Debug, Clone)]
pub enum PriceError {
    InvalidPriceData(String),
    InsufficientData(String),
    CalculationError(String),
}

impl fmt::Display for PriceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PriceError::InvalidPriceData(msg) => write!(f, "Invalid price data: {}", msg),
            PriceError::InsufficientData(msg) => write!(f, "Insufficient data: {}", msg),
            PriceError::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
        }
    }
}

impl std::error::Error for PriceError {}
