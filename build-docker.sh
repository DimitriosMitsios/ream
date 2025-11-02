#!/bin/bash

set -e

echo "Building Docker image from local codebase with risc0 feature..."
echo "NOTE: Building for linux/amd64 platform (risc0 doesn't support ARM64)"

docker build \
  --platform linux/amd64 \
  --build-arg FEATURES="risc0" \
  --build-arg BUILD_PROFILE="release" \
  -t ream-local:latest \
  -f Dockerfile \
  .

echo "Docker image 'ream-local:latest' built successfully!"
echo "You can now use this image in docker-compose.yml"
