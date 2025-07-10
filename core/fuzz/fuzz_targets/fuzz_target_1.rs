#![no_main]
use libfuzzer_sys::fuzz_target;
use smartswap_core::swap_engine::SwapEngine;
use smartswap_core::types::SwapRequest;

fuzz_target!(|data: (String, String, String, String, String)| {
    let (base, quote, amount, price, side) = data;
    let mut engine = SwapEngine::new();
    let req = SwapRequest {
        base,
        quote,
        amount,
        price,
        side,
    };
    let _ = engine.swap(req);
    // Цель — поймать любые паники/краши на некорректных данных
}); 