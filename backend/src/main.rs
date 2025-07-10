mod state;
mod handlers;
mod routes;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use actix_web_prom::PrometheusMetricsBuilder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = state::AppState::new();

    // Инициализация Prometheus метрик
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    println!("SmartSwap backend запущен на http://127.0.0.1:8088");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(prometheus.clone())
            .app_data(actix_web::web::Data::new(app_state.clone()))
            .service(routes::create_routes())
    })
    .bind(("127.0.0.1", 8088))?
    .run()
    .await
}