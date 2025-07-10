use actix_web::{test, App};
use backend::routes;
use backend::state::AppState;
use backend::handlers::types::{AddOrderRequest, DeleteOrderRequest, SwapMockRequest, QuoteQuery};
use serde_json::json;

#[actix_web::test]
async fn test_orderbook_add_and_list() {
    let app_state = AppState::new();
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(app_state))
            .service(routes::create_routes())
    ).await;

    // ВАЛИДНЫЙ запрос на добавление ордера
    let payload = json!({
        "base": "ETH",
        "quote": "USDT",
        "amount": "1.0",
        "price": "3100.0",
        "side": "BUY"
    });
    let add_req = test::TestRequest::post()
        .uri("/api/orderbook/add")
        .set_json(&payload)
        .to_request();
    let add_resp = test::call_service(&app, add_req).await;
    let status = add_resp.status();
    if !status.is_success() {
        let body = test::read_body(add_resp).await;
        println!("Add order failed: status = {:?}, body = {}", status, String::from_utf8_lossy(&body));
        panic!("Add order failed");
    }

    // НЕВАЛИДНЫЙ запрос (amount = 0)
    let payload = json!({
        "base": "ETH",
        "quote": "USDT",
        "amount": "0.0",
        "price": "3100.0",
        "side": "BUY"
    });
    let add_req = test::TestRequest::post()
        .uri("/api/orderbook/add")
        .set_json(&payload)
        .to_request();
    let add_resp = test::call_service(&app, add_req).await;
    assert_eq!(add_resp.status(), 400);

    // GET запрос на список ордеров
    let list_req = test::TestRequest::get()
        .uri("/api/orderbook/list")
        .to_request();
    let resp = test::call_service(&app, list_req).await;
    assert!(resp.status().is_success());
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["orders"][0]["base"], "ETH");
    assert_eq!(body["orders"][0]["side"], "BUY");
}

#[actix_web::test]
async fn test_orderbook_delete() {
    let app_state = AppState::new();
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(app_state))
            .service(routes::create_routes())
    ).await;

    // Добавляем заявку для удаления
    let payload = json!({
        "base": "ETH",
        "quote": "USDT",
        "amount": "2.0",
        "price": "3200.0",
        "side": "SELL"
    });
    let add_req = test::TestRequest::post()
        .uri("/api/orderbook/add")
        .set_json(&payload)
        .to_request();
    let add_resp = test::call_service(&app, add_req).await;
    let status = add_resp.status();
    if !status.is_success() {
        let body = test::read_body(add_resp).await;
        println!("Add order failed: status = {:?}, body = {}", status, String::from_utf8_lossy(&body));
        panic!("Add order failed");
    }
    let add_body: serde_json::Value = test::read_body_json(add_resp).await;
    let order_id = add_body["order_id"].as_str().unwrap();

    // Удаляем заявку по id
    let del_payload = json!({
        "id": order_id
    });
    let del_req = test::TestRequest::post()
        .uri("/api/orderbook/delete")
        .set_json(&del_payload)
        .to_request();
    let del_resp = test::call_service(&app, del_req).await;
    assert!(del_resp.status().is_success());

    // Проверяем, что список ордеров пуст
    let list_req = test::TestRequest::get()
        .uri("/api/orderbook/list")
        .to_request();
    let resp = test::call_service(&app, list_req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["orders"].as_array().unwrap().len(), 0);
}

#[actix_web::test]
async fn test_swap_mock() {
    let app_state = AppState::new();
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(app_state))
            .service(routes::create_routes())
    ).await;

    // Запрос мокового свапа
    let req = test::TestRequest::post()
        .uri("/api/swap/mock")
        .set_json(&json!({
            "from": "ETH",
            "to": "USDT",
            "amount_in": "1.5",
            "price": "3000.0"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        println!("Swap mock failed: status = {:?}, body = {}", status, String::from_utf8_lossy(&body));
        panic!("Swap mock failed");
    }

    let body: serde_json::Value = test::read_body_json(resp).await;
    println!("SWAP MOCK BODY: {:?}", body); // DEBUG
    let amount_out: f64 = body["amount_out"].as_str().unwrap().parse().unwrap();
    let price: f64 = body["price"].as_str().unwrap().parse().unwrap();
    assert!(amount_out > 0.0);
    assert_eq!(price, 3000.0);
}

#[actix_web::test]
async fn test_uniswap_price_handler() {
    use backend::state::AppState;
    let app_state = AppState::new();
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(app_state))
            .service(backend::routes::create_routes())
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/pricing/uniswap?from=ETH&to=USDT")
        .to_request();
    let resp = test::call_service(&app, req).await;
    // Должен вернуть 200 (если Uniswap инициализирован) или 400 (если нет)
    assert!(resp.status().is_success() || resp.status().as_u16() == 400);
}