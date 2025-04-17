FROM rust:latest

# Install Solana tools
RUN set -eux; \
    curl -sSfL --retry 5 --retry-delay 2 https://release.solana.com/v1.19.0/install | sh 
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Install Node.js (LTS version)
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs

# Install dependencies
RUN apt-get update && apt-get install -y curl build-essential git pkg-config libssl-dev libudev-dev

# Install wasm-bindgen
RUN rustup update stable && \
    cargo install wasm-bindgen-cli --version 0.2.88

# Install Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor --tag v0.29.0 anchor-cli

# Set up workspace
WORKDIR /workspace
COPY . .

# Initialize the build
CMD ["anchor", "build"]
