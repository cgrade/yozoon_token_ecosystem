#!/bin/bash

# Build and start the validator
docker-compose up -d validator

# Wait for validator to be ready
echo "Waiting for validator to be ready..."
sleep 5

# Run tests
echo "Running tests..."
docker-compose run --rm tests

# Clean up
echo "Cleaning up..."
docker-compose down 