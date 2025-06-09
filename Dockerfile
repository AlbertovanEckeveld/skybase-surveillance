FROM rust:1.76 as builder


WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ffmpeg libpq5 && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/skybase-surveillance /app/
COPY config.toml /app/
RUN mkdir -p /records

VOLUME ["/records"]
ENV RUST_LOG=info

LABEL authors="Alberto"
ENTRYPOINT ["/app/skybase-surveillance"]