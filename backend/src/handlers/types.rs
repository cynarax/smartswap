use serde::Deserialize;

pub use smartswap_core::types::{AddOrderRequest, DeleteOrderRequest, SwapMockRequest, QuoteQuery};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PriceQuery {
    pub from_token: String,
    pub to_token: String,
}