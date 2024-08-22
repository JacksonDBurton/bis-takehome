FROM rust:1.80.0 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/bis-api bis-api
COPY configuration configuration
ENV APP_ENVIRONMENT production

ENTRYPOINT ["./bis-api"]
