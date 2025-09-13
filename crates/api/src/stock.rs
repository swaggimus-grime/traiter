use serde::{Deserialize, Serialize};
use core::OHLCV;

#[derive(Deserialize, Debug)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum StockWatchReqMsg {
    Subscribe { ticker: String, timeframe: String },
    Unsubscribe { ticker: String, timeframe: String },
    History { ticker: String, timeframe: String, limit: Option<u32> },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum StockWatchResMsg {
    Subscribed { ticker: String, timeframe: String },
    Unsubscribed { ticker: String, timeframe: String },
    Candle { data: OHLCV },
    Candles {
        ticker: String,
        timeframe: String,
        data: Vec<OHLCV>,
    },
    Error { message: String },
}


