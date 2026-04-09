# Build stage
FROM rust:1.77-slim-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
# Create dummy src for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/yatrasathi-backend /app/yatrasathi-backend

# Create data directory for SQLite
RUN mkdir -p /app/data

ENV DATABASE_PATH=/app/data/yatrasathi.db
ENV JWT_SECRET=yatrasathi-dev-secret-2024
ENV SPARROW_SMS_TOKEN=""
ENV SPARROW_SMS_FROM=YatraSathi

EXPOSE 3001

CMD ["./yatrasathi-backend"]
