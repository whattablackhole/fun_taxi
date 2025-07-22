FROM rust:1.88.0 AS build

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    cmake

RUN USER=root cargo new --bin geo_service

WORKDIR /geo_service


COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY .env .

RUN rm ./target/release/deps/geo_service*
RUN cargo build --release

FROM rust:1.88.0

COPY --from=build /geo_service/target/release/geo_service_rs .
COPY --from=build /geo_service/.env .

CMD ["./geo_service_rs"]