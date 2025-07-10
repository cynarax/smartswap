---

# smartswap

![CI](https://github.com/ginfinitylabs/smartswap/actions/workflows/ci.yml/badge.svg)

**smartswap** — production-ready, test-driven DeFi backend for institutional-grade analytics, AMM routing, and automation. Modular, property-based tested, Docker-native, CI/CD, and full observability.

---

## Features

* Modular Rust workspace: core (business logic), backend (API, state, monitoring)
* UniswapV2/AMM price oracles: BSC, ETH, extendable, property/fuzz tested
* Orderbook, swap engine, deterministic math, all types in core
* REST API backend: actix-web, OpenAPI, health, metrics
* Tests: unit, integration, property-based, fuzz, e2e (see /tests, /fuzz)
* Docker-native: docker-compose, Prometheus + Grafana monitoring, CI/CD

---

## Quick Start

* Rust >= 1.74 (stable)
* Docker + docker-compose (for infra)
* Node.js (optional, only for TS API client generation)

* Clone the repo and enter the directory:
  * `git clone https://github.com/ginfinitylabs/smartswap.git`
  * `cd smartswap`

* Build core (with Uniswap support):
  * `cargo build -p smartswap_core --features uniswap`

* Run all tests (unit + e2e + property):
  * `cargo test --workspace`

* Run backend API (dev):
  * `cargo run -p smartswap_backend`

* Bring up monitoring stack (backend + Prometheus + Grafana):
  * `docker-compose up --build`

---

## Project Structure

* smartswap/
  * core/ – DeFi logic: orderbook, price feeds, swaps, types
  * backend/ – REST API, Prometheus, state
  * infra/ – docker-compose, monitoring, test scripts
  * api-client/ – (optional) Generated TS API client (OpenAPI)
  * tests/ – Integration/e2e/property/fuzz tests
  * docker-compose.yml
  * README.md

---

## Modules

* core/orderbook.rs — In-memory orderbook, property-based tests
* core/swap_engine.rs — Swap math, execution logic
* core/price_source.rs — Pluggable price oracles (UniswapV2, Pancake, CoinGecko, mock)
* core/types.rs — Strict types, error models
* backend/main.rs — API entrypoint
* backend/handlers/ — Endpoints, request/response schemas
* backend/routes.rs — Router
* backend/state.rs — Global state (orderbook, price sources)

---

## API

* OpenAPI spec: `api.yaml`
* TypeScript API client: `api-client/` (auto-generated)
* REST endpoints: orderbook, price, swap, health, metrics

---

## Monitoring & CI

* Prometheus + Grafana: infra/grafana, infra/prometheus.yml
* GitHub Actions CI: all tests on push (see .github/workflows/)
* Coverage and quality gates enforced

---

## Contributing

* rustfmt, clippy clean, property-based tests required for critical modules
* PRs accepted only if all tests/lints pass
* All new features must include tests

---

## License

* MIT OR Apache-2.0 (see LICENSE)

---

## Authors

* Artem Chagin / ginfinitylabs
* CYNARA Engineering

---

## Attributions

* Uniswap Labs, PancakeSwap
* actix-web, ethers-rs, proptest, and more

---

*For questions, integration, or enterprise/commercial support: team@dexprobe.xyz*

---
