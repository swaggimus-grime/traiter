use core::market::{Candle, CandleRange};
use tokio::time::{sleep, Duration};
use futures::stream::{self, Stream};
use std::pin::Pin;

/// Replay mode for market data
pub struct DataReplayer {
    bars: CandleRange,
    delay_ms: Option<u64>, // None = no delay (fast-forward), Some = paced
}

impl DataReplayer {
    pub fn new(bars: CandleRange) -> Self {
        Self { bars, delay_ms: None }
    }

    /// Set pacing between bars (simulates real-time).
    /// Example: `set_delay_ms(1000)` → 1 bar per second
    pub fn set_delay_ms(mut self, ms: u64) -> Self {
        self.delay_ms = Some(ms);
        self
    }

    /// Return an async stream of bars
    pub fn into_stream(self) -> Pin<Box<dyn Stream<Item = Candle> + Send>> {
        let delay = self.delay_ms;
        Box::pin(stream::unfold((self.bars, delay, 0), |(bars, delay, idx)| async move {
            if idx >= bars.len() {
                return None;
            }
            let bar = bars[idx].clone();

            if let Some(ms) = delay {
                sleep(Duration::from_millis(ms)).await;
            }

            Some((bar, (bars, delay, idx + 1)))
        }))
    }
}
