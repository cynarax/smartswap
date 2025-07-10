use actix_web::{web, Scope};
use crate::handlers::{
    index, health_check, get_quote, get_price_handler,
    add_order, list_orders, delete_order, swap_mock,
    price_source_handler,
    uniswap_price_handler,
};

pub fn create_routes() -> Scope {
    web::scope("/api")
        .route("/", web::get().to(index))
        .route("/health", web::get().to(health_check))
        .route("/swap/quote", web::get().to(get_quote))
        .route("/swap/mock", web::post().to(swap_mock))
        .route("/pricing/price", web::get().to(get_price_handler))
        .route("/pricing/source", web::get().to(price_source_handler))
        .route("/pricing/uniswap", web::get().to(uniswap_price_handler))
        .route("/orderbook/add", web::post().to(add_order))
        .route("/orderbook/delete", web::post().to(delete_order))
        .route("/orderbook/list", web::get().to(list_orders))
}