mod yahoo;

use anyhow::Result;
use core::market::{Candle, CandleRange};

pub trait Platform {
    fn period(&self) -> Result<CandleRange>;
    fn range(&self, interval: &str, duration: &str) -> Result<CandleRange>;
}

