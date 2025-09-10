mod platforms;
mod replay;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Normalized market bar struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketBar {
    pub ts: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}
