use std::sync::{Arc, Mutex};
use smartswap_core::orderbook::OrderBook;
use smartswap_core::price_source::{PriceSource, MockPriceSource};
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppState {
    pub orderbook: Arc<Mutex<OrderBook>>,
    pub price_source: Arc<dyn PriceSource>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        dotenv().ok();
        // Удалены неиспользуемые переменные и импорты
        Self {
            orderbook: Arc::new(Mutex::new(OrderBook::new())),
            price_source: Arc::new(MockPriceSource),
        }
    }
}