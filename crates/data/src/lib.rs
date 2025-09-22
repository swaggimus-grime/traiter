pub mod providers;
mod replay;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub enum PlayMode {
    Live,
    Backtest {
        start: i64,
        end: i64,
        speed: f64, // 1.0 = real-time, >1 = faster, 0 = instant
    },
}