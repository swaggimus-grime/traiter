
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::{MarketData, Position, Timeframe};
    use crate::ohlcv::OHLCV;

    #[test]
    fn test_ohlcv_creation() {
        let timestamp = Utc::now();
        let ohlcv = OHLCV::new(timestamp, 100.0, 105.0, 98.0, 102.0, 1000.0).unwrap();

        assert_eq!(ohlcv.open, 100.0);
        assert_eq!(ohlcv.close, 102.0);
        assert!(ohlcv.is_bullish());
    }

    #[test]
    fn test_ohlcv_validation() {
        let timestamp = Utc::now();

        // Test invalid high < low
        let result = OHLCV::new(timestamp, 100.0, 98.0, 105.0, 102.0, 1000.0);
        assert!(result.is_err());

        // Test negative prices
        let result = OHLCV::new(timestamp, -100.0, 105.0, 98.0, 102.0, 1000.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_market_data() {
        let mut market_data = MarketData::new("AAPL".to_string(), Timeframe::Day1);
        let timestamp = Utc::now();
        let ohlcv = OHLCV::new(timestamp, 100.0, 105.0, 98.0, 102.0, 1000.0).unwrap();

        market_data.add(ohlcv);
        assert_eq!(market_data.len(), 1);
        assert_eq!(market_data.closes()[0], 102.0);
    }
}