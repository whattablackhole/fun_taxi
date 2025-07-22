FROM rust:1.88.0 AS build

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    cmake

RUN USER=root cargo new --bin driver_service

WORKDIR /driver_service


COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/driver_service*
RUN cargo build --release

FROM rust:1.88.0

COPY --from=build /driver_service/target/release/driver_service_rs .

CMD ["./driver_service_rs"]