mod yahoo;

use anyhow::Result;
use crate::MarketBar;

pub trait Platform {
    fn period(&self) -> Result<Vec<MarketBar>>;
    fn range(&self, interval: &str, duration: &str) -> Result<Vec<MarketBar>>;
}

