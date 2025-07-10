# --- Stage 1: Build ---
FROM rust:latest as builder
WORKDIR /app

# Копируем манифесты для кэширования зависимостей
COPY ../Cargo.toml ../Cargo.lock ./
COPY backend/Cargo.toml ./Cargo.toml
COPY backend/src ./src
COPY ./core ./core
COPY core/Cargo.toml core/Cargo.toml

# Собираем релизную версию backend
RUN cargo build --release -p backend

# --- Stage 2: Minimal runtime ---
FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/backend /app/backend

EXPOSE 8088
CMD ["/app/backend"] 