FROM rust:1.88.0 AS build

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    cmake \
    protobuf-compiler

WORKDIR /geo_service

COPY Cargo.toml Cargo.lock ./
COPY .cargo .cargo

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build

COPY ./src ./src
COPY ./proto ./proto
COPY ./build.rs ./build.rs

RUN cargo build

FROM debian:bookworm-slim AS app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=build /geo_service/target/debug/geo_service_rs .

COPY .env.compose .
COPY .env.kubernetes .
COPY .env.dev .

CMD ["./geo_service_rs"]