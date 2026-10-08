# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.98.1
ARG APP_NAME=axum_postgres_app

# ---------- Build stage ----------
FROM rust:${RUST_VERSION}-slim AS builder
ARG APP_NAME
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Кеш зависимостей через dummy build
COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked && \
    rm -rf src

# Реальная сборка (копируем всё, включая migration)
COPY src ./src
COPY migration ./migration
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked --bin ${APP_NAME} && \
    cp target/release/${APP_NAME} /app/server && \
    strip /app/server

# ---------- Runtime stage ----------
FROM debian:stable-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -g 1001 app && useradd -u 1001 -g app -m app

WORKDIR /app
COPY --from=builder /app/server /app/server
RUN chown app:app /app/server

USER app
EXPOSE 3000
ENTRYPOINT ["/app/server"]