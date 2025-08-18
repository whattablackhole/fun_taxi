FROM rust:1.88.0-slim-bookworm AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    protobuf-compiler \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef

WORKDIR /geo_service

FROM builder AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM builder AS cacher
COPY --from=planner /geo_service/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM builder AS app_builder
WORKDIR /geo_service
COPY --from=cacher /geo_service/target /geo_service/target
COPY --from=cacher /usr/local/cargo/bin /usr/local/cargo/bin
COPY ./src ./src
COPY ./proto ./proto
COPY ./build.rs ./build.rs
COPY Cargo.toml Cargo.lock ./
COPY .cargo .cargo
RUN cargo build --release

FROM debian:bookworm-slim AS app

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=app_builder /geo_service/target/release/geo_service_rs .

CMD ["./geo_service_rs"]