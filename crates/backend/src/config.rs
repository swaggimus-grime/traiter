use dotenvy::dotenv;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub polygon_stocks_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); // load .env if present

        let polygon_stocks_url = env::var("POLYGON_STOCKS_URL")
            .expect("Missing POLYGON_STOCKS_URL");
        let port: u16 = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        Self { polygon_stocks_url, port }
    }
}
