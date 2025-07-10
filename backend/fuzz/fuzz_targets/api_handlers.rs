#![no_main]
use libfuzzer_sys::fuzz_target;
use backend::state::AppState;
use backend::handlers::types::{AddOrderRequest, SwapMockRequest};

fuzz_target!(|data: &[u8]| {
    // Парсим данные как строку для создания запроса
    if let Ok(input_str) = std::str::from_utf8(data) {
        // Разбиваем строку на части
        let parts: Vec<&str> = input_str.split(',').collect();
        
        if parts.len() >= 5 {
            let base = parts[0].to_string();
            let quote = parts[1].to_string();
            let amount = parts[2].to_string();
            let price = parts[3].to_string();
            let side = parts[4].to_string();
            
            // Создаем запрос на добавление ордера
            let req = AddOrderRequest {
                base,
                quote,
                amount,
                price,
                side,
            };
            
            // Тестируем создание состояния приложения
            let app_state = AppState::new();
            
            // Тестируем добавление ордера напрямую в orderbook
            if let Ok(mut orderbook) = app_state.orderbook.lock() {
                if let Ok(id) = orderbook.add_order(req) {
                    // Тестируем удаление ордера
                    let _ = orderbook.delete_order(id);
                }
            };
        }
        
        // Тестируем swap mock запрос
        if parts.len() >= 2 {
            let amount_in = parts[0].to_string();
            let price = parts[1].to_string();
            
            let swap_req = SwapMockRequest {
                amount_in,
                price,
            };
            
            // Просто создаем объект для проверки сериализации
            let _ = serde_json::to_string(&swap_req);
        }
    }
}); 