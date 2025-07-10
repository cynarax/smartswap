# Fuzz-тесты для backend

- **orderbook.rs** — Fuzz логики ордербука (добавление/удаление)
- **swap_engine.rs** — Fuzz логики свапа
- **api_handlers.rs** — Fuzz API-структур и сериализации
- **pricing.rs** — Fuzz модуля ценообразования
- **types.rs** — Fuzz сериализации/десериализации DTO

Все тесты используют best practices: чистые импорты, комментарии, единый стиль, понятные имена. 