use std::fmt;
use std::ops::Index;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::error::PriceError;
use crate::time::{Timeframe, Timestamp};

/// Core OHLCV (Open, High, Low, Close, Volume) data structure
/// This is the fundamental building block for all market data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: Timestamp,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Candle {
    pub fn new(
        timestamp: Timestamp,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> anyhow::Result<Self, PriceError> {
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

        Ok(Candle {
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

impl fmt::Display for Candle {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleRange {
    pub symbol: String,
    pub timeframe: Timeframe,
    pub data: Vec<Candle>,
}

impl CandleRange {
    pub fn new(symbol: String, timeframe: Timeframe) -> Self {
        CandleRange {
            symbol,
            timeframe,
            data: Vec::new(),
        }
    }

    /// Add new OHLCV data point
    pub fn add(&mut self, ohlcv: Candle) {
        self.data.push(ohlcv);
        // Keep data sorted by timestamp
        self.data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }

    /// Get the latest OHLCV data
    pub fn latest(&self) -> Option<&Candle> {
        self.data.last()
    }

    /// Get OHLCV data at specific index from the end
    pub fn get_from_end(&self, index: usize) -> Option<&Candle> {
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

impl Index<usize> for CandleRange {
    type Output = Candle;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IntoIterator for CandleRange {
    type Item = Candle;
    type IntoIter = std::vec::IntoIter<Candle>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}