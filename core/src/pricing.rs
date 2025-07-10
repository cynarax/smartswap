pub use crate::price_source::{PriceSource, MockPriceSource, CoinGeckoPriceSource};
use rust_decimal::Decimal;

#[derive(Debug)]
pub enum PricingError {
    UnknownPair,
}

pub fn get_price(from_token: &str, to_token: &str) -> Result<Decimal, PricingError> {
    match (from_token, to_token) {
        ("ETH", "USDT") => Ok(Decimal::new(3200, 0)),
        ("USDT", "ETH") => Ok(Decimal::ONE / Decimal::new(3200, 0)),
        ("BTC", "USDT") => Ok(Decimal::new(60000, 0)),
        ("USDT", "BTC") => Ok(Decimal::ONE / Decimal::new(60000, 0)),
        _ => Err(PricingError::UnknownPair),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_get_price_eth_usdt() {
        let price = get_price("ETH", "USDT").unwrap();
        assert_eq!(price, Decimal::new(3200, 0));
    }

    #[test]
    fn test_get_price_usdt_eth() {
        let price = get_price("USDT", "ETH").unwrap();
        assert_eq!(price, Decimal::ONE / Decimal::new(3200, 0));
    }

    #[test]
    fn test_get_price_btc_usdt() {
        let price = get_price("BTC", "USDT").unwrap();
        assert_eq!(price, Decimal::new(60000, 0));
    }

    #[test]
    fn test_get_price_unknown_pair() {
        let res = get_price("DOGE", "USDT");
        assert!(matches!(res, Err(PricingError::UnknownPair)));
    }
}