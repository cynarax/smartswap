
# smartswap

![CI](https://github.com/ginfinitylabs/smartswap/actions/workflows/ci.yml/badge.svg)

> **smartswap** — production-ready, test-driven DeFi backend architecture for institutional-grade analytics, routing и автоматизации на AMM (Uniswap, PancakeSwap и др).  
> Модульный, property-based тесты, покрытие всей бизнес-логики, Docker, CI, мониторинг.

---

## Features

- ⚡ **Модульный Rust workspace**: `core` (бизнес-логика), `backend` (API, состояние, мониторинг)
- 🏛️ **UniswapV2/AMM price oracles** (BSC, ETH, extendable, property/fuzz tested)
- 📚 **Orderbook, swap engine, quotes** — строгие типы, вся математика в core
- 🌐 **Actix-web API backend** — REST, OpenAPI, health/metrics
- 🔬 **Тесты**: unit, integration, property-based, fuzz, e2e (см. /tests, /fuzz)
- 🚢 **Docker-native**: docker-compose, Grafana + Prometheus monitoring stack, CI

---

## Быстрый старт

### Зависимости

- Rust >= 1.74 (stable)
- Docker + docker-compose (для инфры)
- Node.js (только для генерации TS API клиента, не обязательно)

### Build & Test

```sh
# Клонируем и заходим в проект
git clone https://github.com/ginfinitylabs/smartswap.git
cd smartswap

# Сборка core (с поддержкой Uniswap)
cargo build -p smartswap_core --features uniswap

# Запуск всех тестов (unit + e2e + property)
cargo test --workspace

# Запуск backend API (dev mode)
cargo run -p smartswap_backend

Docker Compose (мониторинг, prod/dev)

docker-compose up --build
# поднимет backend + Prometheus + Grafana


⸻

Структура проекта

smartswap/
├── core/         # Вся DeFi-логика, ордербук, прайс-фиды, свапы
├── backend/      # REST API (actix-web), прометей, состояние
├── infra/        # docker-compose, мониторинг, тестовые скрипты
├── api-client/   # (опционально) Сгенерённый TS API client (openapi)
├── tests/        # Интеграционные/e2e/property/fuzz тесты
├── docker-compose.yml
└── README.md


⸻

Модули

core
	•	orderbook.rs — In-memory orderbook, property-based тесты
	•	swap_engine.rs — Расчёты свапов, исполнение
	•	price_source.rs — Модули для прайс-оригинов (UniswapV2, Pancake, CoinGecko, mock)
	•	types.rs — Строгие типы, error models

backend
	•	main.rs — Точка входа REST API
	•	handlers/ — Все эндпоинты, схемы запросов/ответов
	•	routes.rs — Роутер
	•	state.rs — Глобальное состояние приложения (ордербук, прайс-сорсы)

⸻

API
	•	OpenAPI-спека: api.yaml
	•	TypeScript API client: api-client/ (генерируется из спеки)
	•	REST endpoints: ордербук, прайс, свап, health, метрики

⸻

Мониторинг & CI
	•	Prometheus + Grafana — infra/grafana, infra/prometheus.yml
	•	GitHub Actions CI:

	•	E2E тесты и отчёты в CI на каждый push (см. workflows/)

⸻

Contributing
	•	Стиль: rustfmt, clippy clean, обязательное property-based тестирование критических модулей
	•	PR — только если проходят все тесты и lint
	•	Любой новый функционал требует теста

⸻

License

MIT OR Apache-2.0
См. LICENSE

⸻

Authors
	•	Артём Чагин / ginfinitylabs
	•	CYNARA Engineering

⸻

Attributions
	•	Uniswap Labs, PancakeSwap
	•	actix-web, ethers-rs, proptest и др.

⸻

Вопросы, интеграция, коммерческая поддержка —
team@dexprobe.xyz

---
