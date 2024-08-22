FROM rust:1.80.0 AS builder

WORKDIR /app
RUN apt-get update && apt-get install lld clang -y
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/bis-api bis-api
COPY configuration configuration
ENV APP_ENVIRONMENT production

ENTRYPOINT ["./bis-api"]
