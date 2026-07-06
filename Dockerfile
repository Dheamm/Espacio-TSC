FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/espacio-tsc ./espacio-tsc
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/static ./static
COPY --from=builder /app/templates ./templates
ENV DATABASE_URL=sqlite:tsc.db
EXPOSE 3000
CMD ["./espacio-tsc"]