FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src ./src
COPY migrations ./migrations
RUN touch src/main.rs && cargo build --release

FROM ubuntu:24.04
WORKDIR /app
COPY --from=builder /app/target/release/space_game_rabbit_consumer .
COPY --from=builder /app/migrations ./migrations

EXPOSE 3120
CMD ["./space_game_rabbit_consumer"]