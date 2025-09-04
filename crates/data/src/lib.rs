mod platforms;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct StockQuote {
    pub date: NaiveDate,
    pub close: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Clone)]
pub struct OptionQuote {
    pub date: NaiveDate,
    pub underlying: String,
    pub expiry: NaiveDate,
    pub strike: f64,
    pub option_type: OptionType,
    pub last_price: f64,
    pub bid: f64,
    pub ask: f64,
    pub volume: Option<u64>,
    pub open_interest: Option<u64>,
}
