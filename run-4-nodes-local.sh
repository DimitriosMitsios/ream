#!/bin/bash

# Script to run 4 lean nodes locally (without Docker) for testing proof generation and gossip
# Each node runs in the background and logs to a separate file

set -e

echo "Starting 4 local lean nodes with proof generation..."
echo "Make sure you have run './setup-local-devnet.sh' first to generate keys!"
echo ""

# Check if release binary exists
if [ ! -f "target/release/ream" ]; then
  echo "Error: Release binary not found at target/release/ream"
  echo "Please build first with: cargo build --features risc0 --release"
  exit 1
fi

REAM_BIN="./target/release/ream"
echo "Using binary: $REAM_BIN"
echo ""

# Kill any existing ream processes
pkill -f "ream.*lean_node" || true
sleep 2

# Clean up old log files
rm -f node0.log node1.log node2.log node3.log

# Get the first node's peer ID after it starts
echo "Starting node0 (bootnode) with RISC0_DEV_MODE=0 (production proofs)..."
RISC0_DEV_MODE=0 $REAM_BIN \
  --proof-gen \
  --ephemeral \
  lean_node \
  --network ephemery \
  --validator-registry-path ./config/validator_registry.yml \
  --node-id ream_0 \
  --private-key-path ./config/node0_key.hex \
  --socket-address 127.0.0.1 \
  --socket-port 9000 \
  --http-address 127.0.0.1 \
  --http-port 5052 \
  --metrics \
  --metrics-address 127.0.0.1 \
  --metrics-port 8080 \
  --bootnodes none \
  > node0.log 2>&1 &

NODE0_PID=$!
echo "Node0 started with PID: $NODE0_PID"

# Wait for node0 to compile and start, then extract peer ID
echo "Waiting for node0 to compile and generate peer ID..."
echo "(This may take 1-2 minutes on first run)"

# Wait up to 3 minutes for the peer ID to appear
TIMEOUT=180
ELAPSED=0
PEER_ID=""

while [ $ELAPSED -lt $TIMEOUT ]; do
  if [ -f node0.log ]; then
    PEER_ID=$(grep -m 1 "local_peer_id" node0.log | grep -oE '16Uiu2[a-zA-Z0-9]+' || echo "")
    if [ -n "$PEER_ID" ]; then
      break
    fi
  fi

  # Check if process is still running
  if ! kill -0 $NODE0_PID 2>/dev/null; then
    echo "Error: Node0 process died. Check node0.log for errors:"
    tail -30 node0.log
    exit 1
  fi

  sleep 2
  ELAPSED=$((ELAPSED + 2))

  # Show progress every 10 seconds
  if [ $((ELAPSED % 10)) -eq 0 ]; then
    echo "  Still waiting... ($ELAPSED seconds)"
  fi
done

if [ -z "$PEER_ID" ]; then
  echo "Failed to get peer ID from node0 after $TIMEOUT seconds. Check node0.log"
  echo "Last 30 lines of node0.log:"
  tail -30 node0.log
  exit 1
fi

echo "Node0 Peer ID: $PEER_ID"
echo ""

# Start node1
echo "Starting node1 with RISC0_DEV_MODE=1 (dev mode)..."
RISC0_DEV_MODE=1 $REAM_BIN \
  --proof-gen \
  --ephemeral \
  lean_node \
  --network ephemery \
  --validator-registry-path ./config/validator_registry.yml \
  --node-id ream_1 \
  --private-key-path ./config/node1_key.hex \
  --socket-address 127.0.0.1 \
  --socket-port 9001 \
  --http-address 127.0.0.1 \
  --http-port 5053 \
  --metrics \
  --metrics-address 127.0.0.1 \
  --metrics-port 8081 \
  --bootnodes /ip4/127.0.0.1/udp/9000/quic-v1/p2p/$PEER_ID \
  > node1.log 2>&1 &

NODE1_PID=$!
echo "Node1 started with PID: $NODE1_PID"

# Start node2
echo "Starting node2 with RISC0_DEV_MODE=1 (dev mode)..."
RISC0_DEV_MODE=1 $REAM_BIN \
  --proof-gen \
  --ephemeral \
  lean_node \
  --network ephemery \
  --validator-registry-path ./config/validator_registry.yml \
  --node-id ream_2 \
  --private-key-path ./config/node2_key.hex \
  --socket-address 127.0.0.1 \
  --socket-port 9002 \
  --http-address 127.0.0.1 \
  --http-port 5054 \
  --metrics \
  --metrics-address 127.0.0.1 \
  --metrics-port 8082 \
  --bootnodes /ip4/127.0.0.1/udp/9000/quic-v1/p2p/$PEER_ID \
  > node2.log 2>&1 &

NODE2_PID=$!
echo "Node2 started with PID: $NODE2_PID"

# Start node3
echo "Starting node3 with RISC0_DEV_MODE=1 (dev mode)..."
RISC0_DEV_MODE=1 $REAM_BIN \
  --proof-gen \
  --ephemeral \
  lean_node \
  --network ephemery \
  --validator-registry-path ./config/validator_registry.yml \
  --node-id ream_3 \
  --private-key-path ./config/node3_key.hex \
  --socket-address 127.0.0.1 \
  --socket-port 9003 \
  --http-address 127.0.0.1 \
  --http-port 5055 \
  --metrics \
  --metrics-address 127.0.0.1 \
  --metrics-port 8083 \
  --bootnodes /ip4/127.0.0.1/udp/9000/quic-v1/p2p/$PEER_ID \
  > node3.log 2>&1 &

NODE3_PID=$!
echo "Node3 started with PID: $NODE3_PID"

echo ""
echo "All 4 nodes started successfully!"
echo ""
echo "Node PIDs:"
echo "  node0: $NODE0_PID"
echo "  node1: $NODE1_PID"
echo "  node2: $NODE2_PID"
echo "  node3: $NODE3_PID"
echo ""
echo "Logs:"
echo "  tail -f node0.log"
echo "  tail -f node1.log"
echo "  tail -f node2.log"
echo "  tail -f node3.log"
echo ""
echo "View all proof generation:"
echo "  tail -f node*.log | grep 'Proof generation'"
echo ""
echo "Stop all nodes:"
echo "  pkill -f 'ream.*lean_node'"
echo ""
echo "APIs:"
echo "  node0: http://127.0.0.1:5052"
echo "  node1: http://127.0.0.1:5053"
echo "  node2: http://127.0.0.1:5054"
echo "  node3: http://127.0.0.1:5055"
