use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum PriceError {
    #[error("Invalid price data: {0}")]
    InvalidPriceData(String),
    #[error("Insufficient data: {0}")]
    InsufficientData(String),
    #[error("Calculation error: {0}")]
    CalculationError(String),
}
