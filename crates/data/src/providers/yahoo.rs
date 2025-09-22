// crates/data/src/providers/yahoo.rs
use crate::providers::{Provider, ProviderStream};
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc, Duration};
use yahoo_finance_api as yahoo;
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use yahoo_finance_api::time::OffsetDateTime;
use dnn_core::market::Candle;
use dnn_core::time::Timestamp;

pub struct Yahoo {
    connector: Arc<yahoo::YahooConnector>,
}

impl Yahoo {
    pub fn new() -> Result<Self> {
        Ok(Self {
            connector: Arc::new(yahoo::YahooConnector::new()?),
        })
    }
}

#[async_trait]
impl Provider for Yahoo {

    async fn stream(&self, symbol: &str, interval: &str) -> Result<ProviderStream> {
        let (tx, rx) = mpsc::unbounded_channel();

        let symbol = symbol.to_string();
        let interval = interval.to_string();
        let connector = self.connector.clone();

        // Poll Yahoo Finance every N seconds
        tokio::spawn(async move {
            loop {
                if let Ok(resp) = connector.get_latest_quotes(&symbol, &interval).await {
                    let quotes = resp.quotes().unwrap();
                    let candles = candles_from_quotes(quotes);
                    for c in candles {
                        let _ = tx.send(c);
                    }
                }
                // Poll every 60s (or match the interval)
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            }
        });

        Ok(UnboundedReceiverStream::new(rx))
    }

    async fn historical(&self, symbol: &str, interval: &str, start: Timestamp, end: Timestamp) -> Result<Vec<Candle>> {
        self.connector.get_quote_history_interval(
            symbol,
            OffsetDateTime::from_unix_timestamp(start.timestamp())?,
            OffsetDateTime::from_unix_timestamp(start.timestamp())?,
            interval
        ).await?.quotes()?.into_iter().map(|q| convert_quote_to_candle(&q, symbol)).collect()
    }
}

/// Convert Yahoo quotes to our Candle format
fn candles_from_quotes(quotes: Vec<yahoo::Quote>) -> Vec<Candle> {
    quotes
        .into_iter()
        .filter_map(|q| convert_quote_to_candle(&q, "UNKNOWN").ok())
        .collect()
}

/// Convert a single Yahoo quote to a Candle
fn convert_quote_to_candle(quote: &yahoo::Quote, symbol: &str) -> Result<Candle> {
    let timestamp = DateTime::from_timestamp(quote.timestamp as i64, 0)
        .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap());

    Candle::new(
        timestamp,
        quote.open,
        quote.high,
        quote.low,
        quote.close,
        quote.volume as f64
    ).map_err(|e| anyhow::anyhow!("Failed to create candle: {}", e))
}