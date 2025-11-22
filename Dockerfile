FROM rust:1.91.1 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

# FROM lukemathwalker/cargo-chef:latest-rust-1.91.1-alpine AS chef

# WORKDIR /app
# RUN apk add --no-cache lld clang

# FROM chef AS planner

# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM chef AS builder
# COPY --from=planner /app/recipe.json recipe.json
# RUN cargo chef cook --release --recipe-path recipe.json
# COPY . .
# ENV SQLX_OFFLINE=true
# RUN cargo build --release --bin velo

FROM debian:bookworm-slim AS runtime

WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/velo velo
COPY configuration configuration
ENV APP_ENVIRONMENT=production
ENTRYPOINT [ "./velo" ]