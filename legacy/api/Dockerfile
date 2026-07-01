FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime libraries
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/spaced_repetition_api /app/

# Expose port 5000 (Dokku's default)
EXPOSE 5000

# Run the binary
CMD ["./spaced_repetition_api"]
