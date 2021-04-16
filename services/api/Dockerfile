FROM rust:slim AS rust
RUN apt-get update && apt-get install -y libpq-dev

FROM rust AS planner
WORKDIR /plan
# We only pay the installation cost once,
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust AS cache-layer
WORKDIR /cache-dependencies
RUN cargo install cargo-chef
COPY --from=planner /plan/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust AS builder
WORKDIR /build
COPY . .
# Copy over the cached dependencies
COPY --from=cache-layer /cache-dependencies/target target
COPY --from=cache-layer /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin api

FROM debian:stable-slim AS runtime
USER root
RUN apt-get update && apt-get -y upgrade\
    && apt-get install -y \
    # Required by diesel_migration.
    libpq-dev \
    ca-certificates \
    && update-ca-certificates
RUN adduser application --system --group --no-create-home
USER application
COPY --from=builder /build/target/release/api /usr/local/bin
CMD ["/usr/local/bin/api"]
