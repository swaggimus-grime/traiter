use serde::{Deserialize, Serialize};
use dnn_core::market::Candle;
use dnn_core::time::Timestamp;

#[derive(Deserialize, Debug)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum StockWatchReqMsg {
    Day { id: String, provider: String, symbol: String, interval: String },
    Night {
        id: String,
        provider: String,
        symbol: String,
        interval: String,
        start: Timestamp,
        end: Timestamp,
        playback_speed: Option<u32>
    },
    Unsubscribe { id: String },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum StockWatchResMsg {
    Candle {
        id: String,
        provider: String,
        symbol: String,
        interval: String,
        candle: Candle,
    },
    Error { message: String },
}


