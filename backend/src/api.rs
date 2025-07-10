//src/api.rs
use actix_web::web;

mod handlers;
pub use handlers::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index))
        .route("/health", web::get().to(health_check))
        .route("/swap/quote", web::get().to(get_quote));
}