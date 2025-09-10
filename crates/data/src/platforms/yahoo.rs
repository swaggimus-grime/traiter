use crate::MarketBar;
use chrono::{DateTime, NaiveDateTime, Utc};
use yahoo_finance_api as yahoo;
use anyhow::Result;
use crate::platforms::Platform;

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
    fn period(&self) -> Result<Vec<MarketBar>> {
        todo!()
    }

    fn range(&self, interval: &str, duration: &str) -> Result<Vec<MarketBar>> {
        todo!()
    }
}

/// Internal helper to convert Yahoo quotes to MarketBar
fn convert_to_bars(quotes: Vec<yahoo::Quote>) -> Vec<MarketBar> {
    quotes
        .into_iter()
        .map(|q| {
            let naive = NaiveDateTime::from_timestamp_opt(q.timestamp as i64, 0)
                .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
            let ts: DateTime<Utc> = DateTime::<Utc>::from_utc(naive, Utc);

            MarketBar {
                ts,
                open: q.open,
                high: q.high,
                low: q.low,
                close: q.close,
                volume: q.volume as f64,
            }
        })
        .collect()
}
