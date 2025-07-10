use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddOrderRequest {
    pub base: String,
    pub quote: String,
    pub amount: String,
    pub price: String,
    pub side: String, // "BUY"/"SELL"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOrderRequest {
    pub id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapMockRequest {
    pub amount_in: String,
    pub price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteQuery {
    pub from_token: String,
    pub to_token: String,
    pub amount_in: String,
    pub price: String,
}