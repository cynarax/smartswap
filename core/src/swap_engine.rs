use rust_decimal::Decimal;
use std::fmt;

pub struct SwapEngine;

#[derive(Debug)]
pub enum SwapError {
    InvalidAmount,
    InvalidPrice,
}

impl fmt::Display for SwapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwapError::InvalidAmount => write!(f, "Invalid amount"),
            SwapError::InvalidPrice => write!(f, "Invalid price"),
        }
    }
}

impl SwapEngine {
    pub fn get_quote(amount_in: Decimal, price: Decimal) -> Result<Decimal, SwapError> {
        if amount_in <= Decimal::ZERO {
            return Err(SwapError::InvalidAmount);
        }
        if price <= Decimal::ZERO {
            return Err(SwapError::InvalidPrice);
        }
        Ok(amount_in * price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_get_quote_valid() {
        let amount_in = dec!(2.0);
        let price = dec!(3.5);
        let result = SwapEngine::get_quote(amount_in, price);
        assert_eq!(result.unwrap(), dec!(7.0));
    }

    #[test]
    fn test_get_quote_invalid_amount() {
        let amount_in = dec!(0.0);
        let price = dec!(3.5);
        let result = SwapEngine::get_quote(amount_in, price);
        assert!(matches!(result, Err(SwapError::InvalidAmount)));
    }

    #[test]
    fn test_get_quote_invalid_price() {
        let amount_in = dec!(2.0);
        let price = dec!(0.0);
        let result = SwapEngine::get_quote(amount_in, price);
        assert!(matches!(result, Err(SwapError::InvalidPrice)));
    }
}