#!/bin/bash

set -e

echo "Cleaning up local devnet..."

# Stop and remove containers
echo "Stopping containers..."
docker-compose down -v

# Remove config files
echo "Removing generated configuration files..."
rm -rf config/

echo ""
echo "Cleanup complete!"
echo ""
echo "All containers stopped, volumes removed, and config files deleted."
