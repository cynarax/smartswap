//! Fuzz-тестирование сериализации/десериализации всех DTO (AddOrderRequest, SwapMockRequest и др.)

#![no_main]
use libfuzzer_sys::fuzz_target;
use backend::handlers::types::{AddOrderRequest, SwapMockRequest};
use serde_json;

fuzz_target!(|data: &[u8]| {
    let _ = serde_json::from_slice::<AddOrderRequest>(data);
    let _ = serde_json::from_slice::<SwapMockRequest>(data);
}); 