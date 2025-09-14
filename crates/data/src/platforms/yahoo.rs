use crate::platforms::CandleRange;
use chrono::{DateTime, NaiveDateTime, Utc};
use yahoo_finance_api as yahoo;
use anyhow::Result;
use crate::platforms::Platform;
use core::market::Candle;

pub struct Yahoo {
    connector: yahoo::YahooConnector,
}

impl Yahoo {
    pub fn new() -> Result<Self> {
        Ok(Self {
            connector: yahoo::YahooConnector::new()?,
        })
    }
}

impl Platform for Yahoo {
    fn period(&self) -> Result<CandleRange> {
        todo!()
    }

    fn range(&self, interval: &str, duration: &str) -> Result<CandleRange> {
        todo!()
    }
}

/// Internal helper to convert Yahoo quotes to OHLCV
fn convert_to_bars(quotes: Vec<yahoo::Quote>) -> Vec<Candle> {
    quotes
        .into_iter()
        .map(|q| {
            let naive = NaiveDateTime::from_timestamp_opt(q.timestamp as i64, 0)
                .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
            let ts: DateTime<Utc> = DateTime::<Utc>::from_utc(naive, Utc);

            Candle::new(ts, q.open, q.high, q.low, q.close, q.volume as f64).unwrap()
        })
        .collect()
}
