use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StockSubscribeMsg {
    pub symbol: String,
}