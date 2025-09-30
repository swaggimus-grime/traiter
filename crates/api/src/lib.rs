pub mod stock;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ProviderType {
    Yahoo,
    Binance,
}

impl From<&str> for ProviderType {
    fn from(s: &str) -> Self {
        match s {
            "yahoo" => ProviderType::Yahoo,
            "binance" => ProviderType::Binance,
            _ => panic!("Unknown provider type: {}", s),
        }
    }
}

impl AsRef<str> for ProviderType {
    fn as_ref(&self) -> &str {
        match self {
            ProviderType::Yahoo => "yahoo",
            ProviderType::Binance => "binance",
        }
    }   
}