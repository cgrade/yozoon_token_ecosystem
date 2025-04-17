FROM rust:1.79.0-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Solana CLI
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.18.26/install)" && \
    export PATH="/root/.local/share/solana/install/active_release/bin:$PATH" && \
    solana --version

# Install Anchor
RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force && \
    avm install 0.29.0 && \
    avm use 0.29.0

# Set working directory
WORKDIR /app

# Copy project files
COPY . .

# Build the program
RUN cargo build && anchor build

# Expose ports
EXPOSE 8899 8900

# Start local validator
CMD ["solana-test-validator", "--reset"]
