use serde::{Deserialize, Serialize};
use dnn_core::market::Candle;
use dnn_core::time::{TimeInterval, Timestamp};
use crate::ProviderType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum StockWatchReqMsg {
    Day { id: String, provider: ProviderType, symbol: String, interval: TimeInterval },
    Night {
        id: String,
        provider: ProviderType,
        symbol: String,
        interval: TimeInterval,
        start: Timestamp,
        end: Timestamp,
        playback_speed: Option<u32>
    },
    Unsubscribe { id: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum StockWatchResMsg {
    Candle {
        id: String,
        provider: ProviderType,
        symbol: String,
        interval: TimeInterval,
        candle: Candle,
    },
    Error { message: String },
}


