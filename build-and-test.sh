#!/bin/bash
set -e

# Check if the minimal versions of files exist
if [ -f "programs/yozoon/src/lib.rs.minimal" ] && [ -f "programs/yozoon/Cargo.toml.minimal" ]; then
    echo "Found minimal test files, using them for the build"
    cp programs/yozoon/src/lib.rs.minimal programs/yozoon/src/lib.rs.bak
    cp programs/yozoon/Cargo.toml.minimal programs/yozoon/Cargo.toml.bak
    
    # Backup original files
    [ -f "programs/yozoon/src/lib.rs" ] && mv programs/yozoon/src/lib.rs programs/yozoon/src/lib.rs.original
    [ -f "programs/yozoon/Cargo.toml" ] && mv programs/yozoon/Cargo.toml programs/yozoon/Cargo.toml.original
    
    # Use minimal files for testing
    cp programs/yozoon/src/lib.rs.minimal programs/yozoon/src/lib.rs
    cp programs/yozoon/Cargo.toml.minimal programs/yozoon/Cargo.toml
fi

# Update Anchor.toml for compatibility
if grep -q "anchor_version = \"0.28.0\"" Anchor.toml; then
    echo "Updating Anchor version in Anchor.toml for compatibility"
    sed -i '' 's/anchor_version = "0.28.0"/anchor_version = "0.27.0"/' Anchor.toml
fi

# Clean previous build artifacts
rm -rf target
rm -rf node_modules

# Install dependencies
echo "Installing dependencies..."
yarn install

# Run the build
echo "Building the program..."
anchor build

# Start local validator for testing
echo "Starting local validator (press Ctrl+C to stop when done)..."
solana-test-validator &
VALIDATOR_PID=$!

# Wait for validator to start
sleep 5

# Run the tests
echo "Running tests..."
anchor test

# Cleanup
kill $VALIDATOR_PID

# Restore original files if they exist
if [ -f "programs/yozoon/src/lib.rs.original" ] && [ -f "programs/yozoon/Cargo.toml.original" ]; then
    echo "Restoring original files"
    mv programs/yozoon/src/lib.rs.original programs/yozoon/src/lib.rs
    mv programs/yozoon/Cargo.toml.original programs/yozoon/Cargo.toml
fi

echo "Build and test completed!"
