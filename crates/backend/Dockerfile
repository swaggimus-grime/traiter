# crates/backend/Dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release --bin backend

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/backend ./backend

# Create a non-root user
RUN useradd -m -u 1001 appuser
USER appuser

EXPOSE 8080
CMD ["./backend"]