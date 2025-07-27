FROM rust:1.88.0 AS build

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    cmake

RUN USER=root cargo new --bin geo_service

WORKDIR /geo_service


COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./.cargo ./.cargo

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/geo_service*
RUN cargo build --release

FROM debian:bookworm-slim AS app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=build /geo_service/target/release/geo_service_rs .

COPY .env.compose .
COPY .env.kubernetes .
COPY .env.dev .

CMD ["./geo_service_rs"]
