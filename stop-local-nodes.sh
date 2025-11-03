#!/bin/bash

# Stop all running ream lean node processes

echo "Stopping all ream lean nodes..."

# Kill all ream lean_node processes
pkill -f "ream.*lean_node"

# Wait a moment for processes to stop
sleep 2

# Check if any processes are still running
if pgrep -f "ream.*lean_node" > /dev/null; then
  echo "Warning: Some processes are still running. Forcing termination..."
  pkill -9 -f "ream.*lean_node"
  sleep 1
fi

echo "All ream lean nodes have been stopped."
echo ""
echo "Log files preserved:"
echo "  node0.log, node1.log, node2.log, node3.log"
echo ""
echo "To restart nodes:"
echo "  ./run-4-nodes-local.sh"
