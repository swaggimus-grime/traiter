
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::market::{CandleRange, Candle};
    use crate::time::TimeInterval;

    #[test]
    fn test_ohlcv_creation() {
        let timestamp = Utc::now();
        let ohlcv = Candle::new(timestamp, 100.0, 105.0, 98.0, 102.0, 1000.0).unwrap();

        assert_eq!(ohlcv.open, 100.0);
        assert_eq!(ohlcv.close, 102.0);
        assert!(ohlcv.is_bullish());
    }

    #[test]
    fn test_ohlcv_validation() {
        let timestamp = Utc::now();

        // Test invalid high < low
        let result = Candle::new(timestamp, 100.0, 98.0, 105.0, 102.0, 1000.0);
        assert!(result.is_err());

        // Test negative prices
        let result = Candle::new(timestamp, -100.0, 105.0, 98.0, 102.0, 1000.0);
        assert!(result.is_err());
    }
    
}