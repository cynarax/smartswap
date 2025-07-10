use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::types::AddOrderRequest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub base: String,
    pub quote: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    pub side: OrderSide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderbookError {
    InvalidAmount,
    InvalidPrice,
    InvalidSide,
    ParseError(String),
}

impl std::fmt::Display for OrderbookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OrderbookError::*;
        match self {
            InvalidAmount => write!(f, "Invalid amount"),
            InvalidPrice => write!(f, "Invalid price"),
            InvalidSide => write!(f, "Invalid side"),
            ParseError(e) => write!(f, "Parse error: {e}"),
        }
    }
}
impl std::error::Error for OrderbookError {}

pub struct OrderBook {
    pub orders: VecDeque<Order>,
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderBook {
    pub fn new() -> Self {
        Self { orders: VecDeque::new() }
    }

    pub fn add_order(&mut self, req: AddOrderRequest) -> Result<Uuid, OrderbookError> {
        let amount = req.amount.parse::<Decimal>().map_err(|e| OrderbookError::ParseError(e.to_string()))?;
        let price = req.price.parse::<Decimal>().map_err(|e| OrderbookError::ParseError(e.to_string()))?;
        let side = match req.side.to_uppercase().as_str() {
            "BUY" => OrderSide::Buy,
            "SELL" => OrderSide::Sell,
            _ => return Err(OrderbookError::InvalidSide),
        };
        if amount <= Decimal::ZERO { return Err(OrderbookError::InvalidAmount); }
        if price <= Decimal::ZERO { return Err(OrderbookError::InvalidPrice); }
        let order = Order {
            id: Uuid::new_v4(),
            base: req.base,
            quote: req.quote,
            amount,
            price,
            side,
        };
        let id = order.id;
        self.orders.push_back(order);
        Ok(id)
    }

    pub fn get_orders(&self) -> &VecDeque<Order> {
        &self.orders
    }

    pub fn delete_order(&mut self, id: Uuid) -> bool {
        if let Some(idx) = self.orders.iter().position(|order| order.id == id) {
            self.orders.remove(idx);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AddOrderRequest;

    fn valid_request(side: &str) -> AddOrderRequest {
        AddOrderRequest {
            base: "ETH".to_string(),
            quote: "USDT".to_string(),
            amount: "1.0".to_string(),
            price: "3000.0".to_string(),
            side: side.to_string(),
        }
    }

    #[test]
    fn test_add_order_valid() {
        let mut ob = OrderBook::new();
        let req = valid_request("BUY");
        let res = ob.add_order(req);
        assert!(res.is_ok());
        assert_eq!(ob.get_orders().len(), 1);
    }

    #[test]
    fn test_add_order_invalid_amount() {
        let mut ob = OrderBook::new();
        let mut req = valid_request("BUY");
        req.amount = "0".to_string();
        let res = ob.add_order(req);
        assert!(matches!(res, Err(OrderbookError::InvalidAmount)));
    }

    #[test]
    fn test_add_order_invalid_price() {
        let mut ob = OrderBook::new();
        let mut req = valid_request("SELL");
        req.price = "0".to_string();
        let res = ob.add_order(req);
        assert!(matches!(res, Err(OrderbookError::InvalidPrice)));
    }

    #[test]
    fn test_add_order_invalid_side() {
        let mut ob = OrderBook::new();
        let req = valid_request("INVALID");
        let res = ob.add_order(req);
        assert!(matches!(res, Err(OrderbookError::InvalidSide)));
    }

    #[test]
    fn test_delete_order() {
        let mut ob = OrderBook::new();
        let req = valid_request("BUY");
        let id = ob.add_order(req).unwrap();
        assert_eq!(ob.get_orders().len(), 1);
        let deleted = ob.delete_order(id);
        assert!(deleted);
        assert_eq!(ob.get_orders().len(), 0);
    }

    #[test]
    fn test_delete_nonexistent_order() {
        let mut ob = OrderBook::new();
        let fake_id = uuid::Uuid::new_v4();
        let deleted = ob.delete_order(fake_id);
        assert!(!deleted);
    }
}