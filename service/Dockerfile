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

# =========
#  Runtime
# =========
FROM debian:bookworm-slim as runtime
WORKDIR /service
USER root

COPY --from=builder /builder/target/release/service /usr/local/bin/service
COPY --from=packages /packages /usr/bin

CMD ["service"]
