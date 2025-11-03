#!/bin/bash

# Generate private keys and validator registry for 4-node local devnet

set -e

echo "Setting up 4-node local devnet..."
echo ""

# Create config directory if it doesn't exist
mkdir -p config

# Generate 4 private keys
echo "Generating private keys for 4 nodes..."
cargo run --bin ream -- generate_private_key --output-path config/node0_key.hex
cargo run --bin ream -- generate_private_key --output-path config/node1_key.hex
cargo run --bin ream -- generate_private_key --output-path config/node2_key.hex
cargo run --bin ream -- generate_private_key --output-path config/node3_key.hex

echo "Generated private keys:"
echo "  - config/node0_key.hex"
echo "  - config/node1_key.hex"
echo "  - config/node2_key.hex"
echo "  - config/node3_key.hex"
echo ""

# Create validator registry
echo "Creating validator registry..."
cat > config/validator_registry.yml << 'EOF'
# Node-based validator registry
# Each node identifier maps to a list of validator IDs it controls

ream_0:
  - 0
ream_1:
  - 1
ream_2:
  - 2
ream_3:
  - 3
EOF

echo "Created validator registry: config/validator_registry.yml"
echo ""
echo "Setup complete! You can now run:"
echo "  ./setup-genesis.sh"
echo "  ./run-4-nodes-local.sh"
