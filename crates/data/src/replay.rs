use tokio_stream::Stream;
use dnn_core::market::Candle;

pub struct ReplayProvider {
    candles: Vec<Candle>,
    speed: f64,
}

impl ReplayProvider {
    pub fn new(candles: Vec<Candle>, speed: f64) -> Self {
        Self { candles, speed }
    }

    pub fn stream(self) -> impl Stream<Item = Candle> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::spawn(async move {
            for window in self.candles.windows(2) {
                let candle = window[0].clone();
                let next = &window[1];

                let _ = tx.send(candle);

                if self.speed > 0.0 {
                    let delta = (next.timestamp - window[0].timestamp).as_seconds_f64();
                    let sleep_ms = delta / self.speed;
                    tokio::time::sleep(std::time::Duration::from_millis(sleep_ms as u64)).await;
                }
            }

            if let Some(last) = self.candles.last().cloned() {
                let _ = tx.send(last);
            }
        });
        tokio_stream::wrappers::UnboundedReceiverStream::new(rx)
    }
}
