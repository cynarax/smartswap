#![no_main]
use libfuzzer_sys::fuzz_target;
use backend::handlers::types::SwapMockRequest;
use smartswap_core::swap_engine::SwapEngine;
use rust_decimal::Decimal;

fuzz_target!(|data: &[u8]| {
    // Парсим данные как строку для создания запроса
    if let Ok(input_str) = std::str::from_utf8(data) {
        // Разбиваем строку на части для создания запроса
        let parts: Vec<&str> = input_str.split(',').collect();
        if parts.len() >= 2 {
            let amount_in = parts[0].to_string();
            let price = parts[1].to_string();
            
            let req = SwapMockRequest {
                amount_in,
                price,
            };
            
            // Тестируем парсинг и вычисления
            if let Ok(amount) = req.amount_in.parse::<Decimal>() {
                if let Ok(price) = req.price.parse::<Decimal>() {
                    let _ = SwapEngine::get_quote(amount, price);
                }
            }
        }
    }
}); 