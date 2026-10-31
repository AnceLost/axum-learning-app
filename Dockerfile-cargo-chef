# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.98.1
ARG APP_NAME=axum_postgres_app

# ---------- Стадия 1: chef ----------
FROM lukemathwalker/cargo-chef:latest-rust-${RUST_VERSION} AS chef
WORKDIR /app

# ---------- Стадия 2: planner ----------
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---------- Стадия 3: builder ----------
FROM chef AS builder
ARG APP_NAME
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked \
        -p ${APP_NAME} --bin ${APP_NAME} \
        -p migration --bin migrator && \
    cp target/release/${APP_NAME} /app/server && \
    cp target/release/migrator /app/migrator && \
    strip /app/server /app/migrator

# ---------- Стадия 4: runtime ----------
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

# ---------- Стадия 5: migrator ----------
FROM debian:stable-slim AS migrator
RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -g 1001 app && useradd -u 1001 -g app -m app

WORKDIR /app
COPY --from=builder /app/migrator /app/migrator
RUN chown app:app /app/migrator

USER app
ENTRYPOINT ["/app/migrator"]