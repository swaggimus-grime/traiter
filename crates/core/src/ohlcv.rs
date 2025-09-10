use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::PriceError;

/// Core OHLCV (Open, High, Low, Close, Volume) data structure
/// This is the fundamental building block for all market data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OHLCV {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl OHLCV {
    pub fn new(
        timestamp: DateTime<Utc>,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Result<Self, PriceError> {
        // Basic validation
        if high < low {
            return Err(PriceError::InvalidPriceData("High price cannot be less than low price".into()));
        }
        if open < 0.0 || high < 0.0 || low < 0.0 || close < 0.0 {
            return Err(PriceError::InvalidPriceData("Prices cannot be negative".into()));
        }
        if volume < 0.0 {
            return Err(PriceError::InvalidPriceData("Volume cannot be negative".into()));
        }

        Ok(OHLCV {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        })
    }

    /// Calculate the typical price (HLC/3)
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }

    /// Calculate the price range (high - low)
    pub fn range(&self) -> f64 {
        self.high - self.low
    }

    /// Calculate returns from previous close
    pub fn returns(&self, previous_close: f64) -> f64 {
        if previous_close == 0.0 {
            0.0
        } else {
            (self.close - previous_close) / previous_close
        }
    }

    /// Check if this is a bullish candle (close > open)
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }

    /// Check if this is a bearish candle (close < open)
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }
}

impl fmt::Display for OHLCV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{}",
            self.timestamp.format("%Y-%m-%d %H:%M"),
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume as u64
        )
    }
}