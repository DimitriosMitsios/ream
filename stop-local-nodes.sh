#!/bin/bash

# Stop all locally running ream nodes

echo "Stopping all ream lean_node processes..."
pkill -f "ream.*lean_node"

echo "Waiting for processes to terminate..."
sleep 2

# Check if any are still running
REMAINING=$(pgrep -f "ream.*lean_node" | wc -l)

if [ $REMAINING -gt 0 ]; then
  echo "Warning: $REMAINING processes still running. Forcing kill..."
  pkill -9 -f "ream.*lean_node"
  sleep 1
fi

echo "All nodes stopped."
echo ""
echo "Log files (node0.log, node1.log, etc.) have been preserved."
echo "To clean up logs: rm -f node*.log"
