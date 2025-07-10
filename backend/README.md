# backend

Веб-сервер на actix-web, реализующий REST API для работы с core.

## Usage

- Запуск локально:
  ```sh
  cargo run -p backend
  ```
- Переменные окружения можно задать через `.env`.
- Для сборки Docker:
  ```sh
  docker build -t backend .
  docker run --env-file .env -p 8080:8080 backend
  ```

## Coverage

- REST API для работы с ордербуком и ценами
- Глобальное состояние (AppState)
- Интеграция с core (orderbook, price_source)
- Метрики Prometheus, CORS, dotenv
- Тесты: см. tests/

## Dev Notes

- Основная логика — в handlers/, state.rs, routes.rs
- Для разработки: `cargo watch -x run -p backend`
- Для тестирования: `cargo test -p backend`
- Все импорты и зависимости согласованы с core
- Код чистый по clippy 