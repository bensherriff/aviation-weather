# =========
#  Builder
# =========
FROM rust:bookworm as builder
WORKDIR /builder

COPY migrations ./migrations
COPY src ./src
COPY Cargo.toml ./

RUN apt-get update && apt-get install -y cmake
RUN cargo build --release

# ======
#  Keys
# ======
FROM debian:bookworm-slim as keys
WORKDIR /keys

RUN apt-get update && apt-get install -y openssl libpq-dev
RUN openssl genrsa -out access.pem 4096
RUN openssl rsa -in access.pem -pubout -outform PEM -out access.pem.pub
RUN openssl genrsa -out refresh.pem 4096
RUN openssl rsa -in refresh.pem -pubout -outform PEM -out refresh.pem.pub

# =========
#  Runtime
# =========
FROM keys as runtime
WORKDIR /service
USER root

COPY --from=builder /builder/target/release/service /usr/local/bin/service
COPY --from=keys /keys /keys

CMD ["service"]
