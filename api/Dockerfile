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

# ======
#  Keys
# ======
FROM debian:bookworm-slim AS keys
WORKDIR /keys

RUN apt-get update && apt-get install -y openssl libpq-dev
RUN openssl genrsa -out access.pem 4096
RUN openssl rsa -in access.pem -pubout -outform PEM -out access.pem.pub
RUN openssl genrsa -out refresh.pem 4096
RUN openssl rsa -in refresh.pem -pubout -outform PEM -out refresh.pem.pub

# =========
#  Runtime
# =========
FROM keys AS runtime
WORKDIR /api
USER root

COPY --from=builder /builder/target/release/api /usr/local/bin/api
COPY --from=keys /keys /keys

CMD ["api"]
