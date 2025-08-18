FROM rust:1.88.0 AS build

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    cmake \
    protobuf-compiler

RUN USER=root cargo new --bin streamgate

WORKDIR /streamgate

COPY ./axum-tonic ./axum-tonic
COPY ./Cargo.lock .
COPY ./Cargo.toml .


RUN CARGO_HTTP_MULTIPLEXING=false cargo build --release
RUN rm src/*.rs


COPY ./src ./src
COPY ./proto ./proto
COPY ./build.rs ./build.rs

RUN rm ./target/release/deps/streamgate*
RUN cargo build --release

FROM debian:bookworm-slim AS app

COPY --from=build /streamgate/target/release/streamgate_rs .
COPY .env.compose .
COPY .env.kubernetes .
COPY .env.dev .


CMD ["./streamgate_rs"]