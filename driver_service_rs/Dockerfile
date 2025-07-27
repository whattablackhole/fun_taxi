FROM rust:1.88.0 AS build

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    cmake

RUN USER=root cargo new --bin driver_service

WORKDIR /driver_service


COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./.cargo ./.cargo


RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/driver_service*
RUN cargo build --release

FROM debian:bookworm-slim AS app

COPY --from=build /driver_service/target/release/driver_service_rs .
COPY .env.compose .
COPY .env.kubernetes .
COPY .env.dev .

CMD ["./driver_service_rs"]