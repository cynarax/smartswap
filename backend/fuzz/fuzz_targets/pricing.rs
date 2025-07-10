//! Fuzz-тестирование модуля ценообразования: get_price с разными токенами.

#![no_main]
use libfuzzer_sys::fuzz_target;
use smartswap_core::pricing::get_price;

fuzz_target!(|data: &[u8]| {
    if let Ok(input_str) = std::str::from_utf8(data) {
        let parts: Vec<&str> = input_str.split(',').collect();
        if parts.len() >= 2 {
            let _ = get_price(parts[0], parts[1]);
        }
    }
}); 