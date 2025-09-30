use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type Timestamp = DateTime<Utc>;

/// Represents different timeframes for market data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeInterval {
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

impl TimeInterval {
    /// Convert timeframe to seconds
    pub fn to_seconds(&self) -> i64 {
        match self {
            TimeInterval::Minute1 => 60,
            TimeInterval::Minute5 => 300,
            TimeInterval::Minute15 => 900,
            TimeInterval::Minute30 => 1800,
            TimeInterval::Hour1 => 3600,
            TimeInterval::Hour4 => 14400,
            TimeInterval::Day1 => 86400,
            TimeInterval::Week1 => 604800,
            TimeInterval::Month1 => 2592000, // Approximate
        }
    }
}

impl fmt::Display for TimeInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TimeInterval::Minute1 => "1m",
            TimeInterval::Minute5 => "5m",
            TimeInterval::Minute15 => "15m",
            TimeInterval::Minute30 => "30m",
            TimeInterval::Hour1 => "1h",
            TimeInterval::Hour4 => "4h",
            TimeInterval::Day1 => "1d",
            TimeInterval::Week1 => "1w",
            TimeInterval::Month1 => "1M",
        };
        write!(f, "{}", s)
    }
}