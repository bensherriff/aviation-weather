FROM rust:1.72.1-bookworm

WORKDIR /service
USER root

COPY migrations ./migrations
COPY src ./src
COPY airport-codes.json Cargo.toml diesel.toml ./

RUN cargo build --release

CMD ["./target/release/weather-service"]