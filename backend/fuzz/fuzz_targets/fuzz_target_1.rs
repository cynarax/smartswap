#![no_main]
use libfuzzer_sys::fuzz_target;
use smartswap_core::swap_engine::SwapEngine;
use rust_decimal::Decimal;

fuzz_target!(|data: (String, String)| {
    // Пробуем распарсить строки в Decimal
    if let (Ok(amount_in), Ok(price)) = (data.0.parse::<Decimal>(), data.1.parse::<Decimal>()) {
        let _ = SwapEngine::get_quote(amount_in, price);
    }
    // Цель — поймать любые паники/краши на некорректных данных
}); 