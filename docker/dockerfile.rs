FROM rust:latest AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    curl \
    git

# Set up the working directory
WORKDIR /app

# Copy the Rust source code
COPY . .

# Build the Rust project in release mode
RUN cargo build --release

# Create a minimal runtime image
FROM debian:bullseye-slim

# Set environment variables
ENV NODE_ENV production

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/bitcoinz-node /usr/local/bin/bitcoinz-node

# Add configuration and data volumes
VOLUME ["/config", "/data", "/logs"]

# Expose RPC and P2P ports
EXPOSE 8332 8333

# Set the command to start the node
CMD ["/usr/local/bin/bitcoinz-node", "--config", "/config/bitcoinz.conf"]
