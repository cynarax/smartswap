# core

Библиотека бизнес-логики для backend.

## Usage

Добавьте зависимость в Cargo.toml:

```toml
[dependencies]
smartswap_core = { path = "../core", features = ["uniswap"] }
```

Импортируйте нужные модули:

```rust
use smartswap_core::orderbook::OrderBook;
use smartswap_core::price_source::{PriceSource, MockPriceSource};
```

## Build с фичами

- Сборка с поддержкой Uniswap:
  ```sh
  cargo build -p smartswap_core --features uniswap
  ```
- Сборка без Uniswap (минимальный набор):
  ```sh
  cargo build -p smartswap_core --no-default-features
  ```

## Coverage

- Ордербук (OrderBook)
- Источники цен: CoinGecko, Uniswap (feature), Mock
- Логика обмена, расчёты, типы
- Тесты: property-based, fuzzing (см. tests/ и fuzz/)

## Dev Notes

- Для поддержки Uniswap используйте feature `uniswap`.
- Все внешние зависимости указаны в Cargo.toml.
- Для тестирования: `cargo test -p smartswap_core`, для fuzzing: `cargo fuzz run ...`
- Код покрыт clippy, неиспользуемый код разрешён только явно. 