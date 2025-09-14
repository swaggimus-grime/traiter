use serde::{Deserialize, Serialize};
use core::market::Candle;
use core::time::Timestamp;

#[derive(Deserialize, Debug)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum StockWatchReqMsg {
    Subscribe { ticker: String, timestamp: Timestamp },
    Unsubscribe { ticker: String, timestamp: Timestamp },
    History { ticker: String, timestamp: Timestamp, limit: Option<u32> },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum StockWatchResMsg {
    Subscribed { ticker: String, timestamp: Timestamp },
    Unsubscribed { ticker: String, timestamp: Timestamp },
    Candle { data: Candle },
    Candles {
        ticker: String,
        timestamp: Timestamp,
        data: Vec<Candle>,
    },
    Error { message: String },
}


