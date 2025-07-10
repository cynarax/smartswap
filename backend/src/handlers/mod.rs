use actix_web::{web, HttpResponse, Responder};
use crate::state::AppState;
use self::types::{AddOrderRequest, DeleteOrderRequest, SwapMockRequest, QuoteQuery};
use smartswap_core::swap_engine::SwapEngine;
use smartswap_core::pricing;
use smartswap_core::price_source::PriceSource;

pub mod types;

#[derive(serde::Deserialize)]
pub struct PriceSourceQuery {
    pub from: String,
    pub to: String,
    pub amount: Option<String>,
}

// --- Healthcheck ---
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

// --- Hello world index ---
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "SmartSwap backend live" }))
}

// --- Добавить ордер ---
pub async fn add_order(
    data: web::Data<AppState>,
    payload: web::Json<AddOrderRequest>,
) -> impl Responder {
    let mut ob = data.orderbook.lock().unwrap();
    match ob.add_order(payload.into_inner()) {
        Ok(order_id) => HttpResponse::Ok().json(serde_json::json!({ "order_id": order_id, "status": "ok" })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e.to_string() })),
    }
}

// --- Получить список ордеров ---
pub async fn list_orders(
    data: web::Data<AppState>
) -> impl Responder {
    let ob = data.orderbook.lock().unwrap();
    let orders = ob.get_orders();
    HttpResponse::Ok().json(serde_json::json!({ "orders": orders }))
}

// --- Удалить ордер по id ---
pub async fn delete_order(
    data: web::Data<AppState>,
    payload: web::Json<DeleteOrderRequest>,
) -> impl Responder {
    let mut ob = data.orderbook.lock().unwrap();
    if ob.delete_order(payload.id) {
        HttpResponse::Ok().json(serde_json::json!({ "status": "deleted" }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({ "error": "Order not found" }))
    }
}

// --- Swap mock endpoint ---
pub async fn swap_mock(
    payload: web::Json<SwapMockRequest>,
) -> impl Responder {
    use rust_decimal::Decimal;
    let amount_in = payload.amount_in.parse::<Decimal>();
    let price = payload.price.parse::<Decimal>();
    match (amount_in, price) {
        (Ok(amount_in), Ok(price)) if amount_in > Decimal::ZERO && price > Decimal::ZERO => {
            let amount_out = amount_in * price;
            HttpResponse::Ok().json(serde_json::json!({
                "amount_out": amount_out.to_string(),
                "price": payload.price
            }))
        },
        (Err(_), _) | (_, Err(_)) => {
            HttpResponse::BadRequest().json(serde_json::json!({ "error": "Invalid number format" }))
        },
        _ => {
            HttpResponse::BadRequest().json(serde_json::json!({ "error": "Invalid amount or price" }))
        }
    }
}

// --- Получить quote (расчет без добавления заявки) ---
pub async fn get_quote(
    query: web::Query<QuoteQuery>,
) -> impl Responder {
    use rust_decimal::Decimal;
    let amount_in = query.amount_in.parse::<Decimal>();
    let price = query.price.parse::<Decimal>();
    match (amount_in, price) {
        (Ok(amount_in), Ok(price)) if amount_in > Decimal::ZERO && price > Decimal::ZERO => {
            match SwapEngine::get_quote(amount_in, price) {
                Ok(amount_out) => HttpResponse::Ok().json(serde_json::json!({
                    "amount_out": amount_out,
                    "price": query.price,
                    "from_token": query.from_token,
                    "to_token": query.to_token,
                })),
                Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e.to_string() })),
            }
        },
        (Err(_), _) | (_, Err(_)) => {
            HttpResponse::BadRequest().json(serde_json::json!({ "error": "Invalid number format" }))
        },
        _ => {
            HttpResponse::BadRequest().json(serde_json::json!({ "error": "Invalid amount or price" }))
        }
    }
}

// --- Legacy: статический прайсинг (core) ---
pub async fn get_price_handler(
    query: web::Query<PriceSourceQuery>,
) -> impl Responder {
    match pricing::get_price(&query.from, &query.to) {
        Ok(price) => HttpResponse::Ok().json(serde_json::json!({ "price": price })),
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({ "error": "Unknown pair" })),
    }
}

// --- Новый handler: динамический прайсинг через AppState::price_source (Uniswap/Mock/CoinGecko) ---
pub async fn price_source_handler(
    data: web::Data<AppState>,
    query: web::Query<PriceSourceQuery>,
) -> impl Responder {
    let amount = query.amount.as_deref();
    match data.price_source.get_price(&query.from, &query.to, amount).await {
        Ok(price) => HttpResponse::Ok().json(serde_json::json!({ "price": price })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}

// --- Получить цену только с Uniswap ---
pub async fn uniswap_price_handler(
    data: web::Data<AppState>,
    query: web::Query<PriceSourceQuery>,
) -> impl Responder {
    // Попробуем привести price_source к UniswapV2PriceSource
    use smartswap_core::price_source::uniswap_v2::UniswapV2PriceSource;
    let price_source = data.price_source.clone();
    if let Some(uniswap) = price_source.as_any().downcast_ref::<UniswapV2PriceSource>() {
        let amount = query.amount.as_deref();
        match uniswap.get_price(&query.from, &query.to, amount).await {
            Ok(price) => HttpResponse::Ok().json(serde_json::json!({ "price": price, "source": "uniswap" })),
            Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
        }
    } else {
        HttpResponse::BadRequest().json(serde_json::json!({ "error": "UniswapV2PriceSource не инициализирован" }))
    }
}