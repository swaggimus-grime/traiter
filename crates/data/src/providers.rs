mod yahoo;

use std::pin::Pin;
pub use yahoo::Yahoo;

use anyhow::Result;
use async_trait::async_trait;
use yahoo_finance_api::time::OffsetDateTime;
use futures::{StreamExt as _, Stream, stream};
use tokio_stream::wrappers::UnboundedReceiverStream;
use api::ProviderType;
use dnn_core::market::Candle;
use dnn_core::time::{TimeInterval, Timestamp};

pub type ProviderStream = UnboundedReceiverStream<Candle>;

#[async_trait]
pub trait Provider {
    async fn stream(&self, symbol: &str, interval: TimeInterval) -> anyhow::Result<ProviderStream>;

    async fn historical(
        &self,
        symbol: &str,
        interval: TimeInterval,
        start: Timestamp,
        end: Timestamp
    ) -> anyhow::Result<Vec<Candle>>;
    
    fn get_type(&self) -> ProviderType;
}