#!/bin/bash

# Setup shared genesis configuration for all nodes
# This ensures all nodes start with the EXACT same genesis time

set -e

echo -e "\033[0;32mSetting up local genesis for 4 ream nodes...\033[0m"
echo -e "\033[1;33mCreating directories...\033[0m"

# Create genesis directory
mkdir -p genesis

# Set genesis time to 60 seconds from now to give time for building/starting nodes
GENESIS_TIME=$(($(date +%s) + 60))
GENESIS_DATE=$(date -r $GENESIS_TIME "+%Y-%m-%d %H:%M:%S")

echo "Genesis time set to: $GENESIS_DATE (60 seconds from now)"
echo -e "\033[1;33mGenerating shared genesis config...\033[0m"

# Generate genesis config that all nodes will use
cat > genesis/config.yaml << EOF
# Lean Chain Genesis Configuration
# All nodes must use this EXACT config for consensus

GENESIS_TIME: ${GENESIS_TIME}
JUSTIFICATION_LOOKBACK_SLOTS: 3
SECONDS_PER_SLOT: 4
VALIDATOR_COUNT: 4
EOF

echo -e "\033[0;32mGenesis config created at genesis/config.yaml\033[0m"
echo ""
echo "Configuration:"
echo "  Genesis Time: ${GENESIS_TIME} (${GENESIS_DATE})"
echo "  Justification Lookback: 3 slots"
echo "  Seconds Per Slot: 4"
echo "  Validator Count: 4"
echo ""
echo -e "\033[0;32mSetup complete!\033[0m"
echo "Next steps:"
echo "  1. Keys already generated in config/"
echo "  2. Run: ./run-4-nodes-local.sh"
