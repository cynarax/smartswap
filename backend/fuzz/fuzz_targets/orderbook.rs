//! Fuzz-тестирование логики ордербука с использованием Arbitrary.

#![no_main]
use libfuzzer_sys::fuzz_target;
use backend::state::AppState;
use backend::handlers::types::AddOrderRequest;
use arbitrary::Arbitrary;
mod arbitrary_types;
use arbitrary_types::FuzzAddOrderRequest;

fuzz_target!(|data: &[u8]| {
    if let Ok(fuzz_req) = FuzzAddOrderRequest::arbitrary(&mut arbitrary::Unstructured::new(data)) {
        let req: AddOrderRequest = fuzz_req.into();
        let app_state = AppState::new();
        if let Ok(mut orderbook) = app_state.orderbook.lock() {
            let _ = orderbook.add_order(req);
        };
    }
}); 