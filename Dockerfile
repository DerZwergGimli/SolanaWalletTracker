FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin solanaWalletTracker

# We do need the following deps!
FROM debian:bullseye-slim AS runtime
RUN apt-get update
RUN apt-get install openssl -y
RUN apt-get install curl -y

WORKDIR app
RUN mkdir /usr/local/bin/files
COPY files/* /usr/local/bin/files/
COPY --from=builder /app/target/release/solanaWalletTracker /usr/local/bin
RUN chmod +x /usr/local/bin/solanaWalletTracker
WORKDIR /usr/local/bin
ENTRYPOINT ["/usr/local/bin/solanaWalletTracker"]