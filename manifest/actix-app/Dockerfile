FROM lukemathwalker/cargo-chef:latest-rust-1.69.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin actix_app

# We do not need the Rust toolchain to run the binary!
# FROM debian:buster-slim AS runtime
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/actix_app /usr/local/bin

ENTRYPOINT ["/usr/local/bin/actix_app"]

# ---------------------------------------------------------------
# Image on Docker Hub: https://hub.docker.com/r/glebcom/actix-app
# Image size:
#   - 1.0.0-bullseye-slim: 35.04 MB
# ---------------------------------------------------------------