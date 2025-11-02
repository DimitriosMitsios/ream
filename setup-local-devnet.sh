#!/bin/bash

set -e

echo "Setting up local 4-node devnet..."

# Create config directory
mkdir -p config

# Generate private keys for each node
echo "Generating private keys for 4 nodes..."
for i in {0..3}; do
    echo "Generating key for node$i..."
    # Extract only the hex key (first 64 characters) from the output
    docker run --rm ream-local:latest generate_private_key --output-path /dev/stdout 2>&1 | grep -o '^[a-f0-9]\{64\}' > config/node${i}_key.hex
    echo "Node $i key (first 20 chars): $(head -c 20 config/node${i}_key.hex)"
done

echo ""
echo "Note: Nodes will discover each other automatically through the validator registry."

# Create validator registry
# Format: node_id maps to list of validator IDs
echo ""
echo "Creating validator registry..."
cat > config/validator_registry.yml <<EOF
ream_0:
    - 0
ream_1:
    - 1
ream_2:
    - 2
ream_3:
    - 3
EOF


echo ""
echo "Setup complete!"
echo ""
echo "Configuration files created:"
echo "  - config/node0_key.hex"
echo "  - config/node1_key.hex"
echo "  - config/node2_key.hex"
echo "  - config/node3_key.hex"
echo "  - config/validator_registry.yml"
echo ""
echo "To start the network, run:"
echo "  docker-compose up -d"
echo ""
echo "To view logs:"
echo "  docker-compose logs -f"
echo ""
echo "To stop the network:"
echo "  docker-compose down"
