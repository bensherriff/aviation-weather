# =========
#  Builder
# =========
FROM rust:bookworm AS builder
WORKDIR /builder

COPY migrations ./migrations
COPY src ./src
COPY Cargo.toml ./

RUN apt-get update && apt-get install -y cmake
RUN cargo build --release

# =========
#  Runtime
# =========
FROM debian:bookworm-slim AS runtime
WORKDIR /api
RUN apt-get update && apt-get install -y openssl libpq-dev ca-certificates
USER root

COPY --from=builder /builder/target/release/api /usr/local/bin/api

CMD ["api"]
