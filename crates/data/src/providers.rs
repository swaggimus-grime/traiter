mod yahoo;

use std::pin::Pin;
pub use yahoo::Yahoo;

use anyhow::Result;
use async_trait::async_trait;
use yahoo_finance_api::time::OffsetDateTime;
use futures::{StreamExt as _, Stream, stream};
use tokio_stream::wrappers::UnboundedReceiverStream;
use dnn_core::market::Candle;
use dnn_core::time::Timestamp;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ProviderType {
    Yahoo,
    Binance,
}

impl From<&str> for ProviderType {
    fn from(s: &str) -> Self {
        match s {
            "yahoo" => ProviderType::Yahoo,
            "binance" => ProviderType::Binance,
            _ => panic!("Unknown provider type: {}", s),
        }
    }
}

pub type ProviderStream = UnboundedReceiverStream<Candle>;

#[async_trait]
pub trait Provider {
    async fn stream(&self, symbol: &str, interval: &str) -> anyhow::Result<ProviderStream>;

    async fn historical(
        &self,
        symbol: &str,
        interval: &str,
        start: Timestamp,
        end: Timestamp
    ) -> anyhow::Result<Vec<Candle>>;
}