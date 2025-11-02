# Local 4-Node Devnet Setup

This guide provides two ways to run 4 Ream lean nodes locally using your **current local codebase**:

1. **Docker Compose** - Containerized environment (easier setup, but production proofs may not work due to emulation)
2. **Direct on Host** - Run directly on your machine (uses real hardware, production proofs work)

## Prerequisites

### For Docker Setup
- Docker installed and running
- Docker Compose installed

### For Direct/Host Setup
- Rust toolchain installed
- risc0 toolchain installed (`curl -L https://risczero.com/install | bash && rzup install`)

## Quick Start - Docker Method

### 1. Build the Docker Image

First, build a Docker image from your current codebase with the risc0 feature enabled:

```bash
./build-docker.sh
```

This creates a Docker image called `ream-local:latest` from your current code.

### 2. Setup the Network

Generate private keys and validator registry:

```bash
./setup-local-devnet.sh
```

This will:
- Generate 4 private keys (one for each node)
- Create a validator registry with 4 validators
- Configure the network topology

### 3. Start All Nodes

```bash
docker-compose up -d
```

### 4. View Logs

To view logs from all nodes:

```bash
docker-compose logs -f
```

To view logs from a specific node:

```bash
docker-compose logs -f node0
docker-compose logs -f node1
docker-compose logs -f node2
docker-compose logs -f node3
```

### 5. Stop the Network

```bash
docker-compose down
```

### 6. Clean Up Everything

To stop all nodes and remove all generated files:

```bash
./cleanup-devnet.sh
```

## Network Configuration

The devnet runs 4 nodes with the following configuration:

| Node | P2P Port | HTTP API Port | Metrics Port | IP Address  | Validator ID | Peer ID (for reference) |
|------|----------|---------------|--------------|-------------|--------------|-------------------------|
| node0| 9000     | 5052          | 8080         | 172.20.0.10 | ream_0       | 16Uiu2HAmKMRmdX3M7Dc2bxVN4CQXAH9kQHw93ajiyGVqd63LY8kK |
| node1| 9001     | 5053          | 8081         | 172.20.0.11 | ream_1       | Auto-generated          |
| node2| 9002     | 5054          | 8082         | 172.20.0.12 | ream_2       | Auto-generated          |
| node3| 9003     | 5055          | 8083         | 172.20.0.13 | ream_3       | Auto-generated          |

- **Network**: Ephemery
- **Subnet**: 172.20.0.0/24
- **Proof Generation**: Enabled with `--proof-gen` flag
  - Docker: Uses `RISC0_DEV_MODE=1` (dev mode proofs due to emulation issues)
  - Direct/Host: Uses real ZK proofs (production mode)
- **Bootnode**: node0 acts as the bootnode for nodes 1-3
- **Peer Discovery**: Nodes 1-3 connect to node0 via its multiaddr

## Accessing APIs

Once the nodes are running, you can access their APIs:

```bash
# Check node0 status
curl http://localhost:5052/eth/v1/node/health

# Check node1 status
curl http://localhost:5053/eth/v1/node/health

# Check node2 status
curl http://localhost:5054/eth/v1/node/health

# Check node3 status
curl http://localhost:5055/eth/v1/node/health
```

## Verifying Gossip Communication

After the nodes start, you should see logs indicating:
- **Peers connecting to each other** - "Connected to peer" messages
- **Blocks being proposed and gossiped** - "Broadcasted block" messages
- **Votes being gossiped** - "Broadcasted vote" messages
- **Votes being received from other validators** - "Processing vote by Validator X" messages
- Nodes receiving blocks and votes from each other via gossip

### Current Status
✅ **Peer discovery is working** - All nodes successfully connect to each other
✅ **Block gossip is working** - Blocks are being proposed and broadcasted
✅ **Vote gossip is working** - Votes are being exchanged between nodes
✅ **Block proof generation** - Enabled with risc0 prover

## Known Issues

### 1. Hard-coded Peer ID

**Issue**: node0's peer ID is hard-coded in docker-compose.yml

**Impact**: If you regenerate node0's private key, you'll need to:
1. Start node0 first
2. Extract the new peer ID from logs: `docker logs ream-node0 2>&1 | grep local_peer_id`
3. Update docker-compose.yml with the new peer ID in the `--bootnodes` parameter for nodes 1-3

**Future Improvement**: Could automate this with a script that:
1. Starts node0
2. Extracts peer ID
3. Dynamically updates docker-compose.yml or uses environment variables

## Rebuilding After Code Changes

If you make changes to your code:

1. Rebuild the Docker image:
   ```bash
   ./build-docker.sh
   ```

2. Restart the nodes:
   ```bash
   docker-compose down
   docker-compose up -d
   ```

Note: You don't need to run `setup-local-devnet.sh` again unless you want to regenerate keys.

## Troubleshooting

### Nodes can't connect to each other

- Check that all containers are running: `docker ps`
- Check the logs for connection errors: `docker-compose logs -f`
- Ensure the Docker network is created: `docker network ls | grep ream`
- Verify that node0's peer ID in docker-compose.yml matches the actual peer ID from node0's logs

### "InsufficientPeers" errors

- This is expected during the first few seconds as nodes discover each other
- If it persists:
  - Check that node0 (the bootnode) is running properly
  - Verify the peer ID in docker-compose.yml matches node0's actual peer ID
  - Check node0's logs: `docker logs ream-node0`

### Want to start fresh

Run the cleanup script and start over:
```bash
./cleanup-devnet.sh
./setup-local-devnet.sh
docker-compose up -d
```

---

## Quick Start - Direct/Host Method (Recommended for Production Proofs)

This method runs nodes directly on your machine, which:
- Uses your real CPU (no emulation issues)
- Generates real ZK proofs (not just dev mode)
- Better performance for proof generation

### 1. Setup the Network

Generate private keys and validator registry (same as Docker):

```bash
./setup-local-devnet.sh
```

### 2. Start All 4 Nodes

```bash
./run-4-nodes-local.sh
```

This will:
- Start 4 nodes in the background
- Each node logs to its own file (node0.log, node1.log, etc.)
- Use your machine's real CPU for proof generation
- Enable actual ZK proofs (not dev mode)

### 3. View Logs

To view logs from all nodes:

```bash
tail -f node*.log
```

To view logs from a specific node:

```bash
tail -f node0.log
tail -f node1.log
tail -f node2.log
tail -f node3.log
```

To view only proof generation:

```bash
tail -f node*.log | grep "Proof generation"
```

### 4. Stop All Nodes

```bash
./stop-local-nodes.sh
```

Or manually:

```bash
pkill -f "ream.*lean_node"
```

### 5. Clean Up

To remove log files:

```bash
rm -f node*.log
```

To also remove generated keys:

```bash
./cleanup-devnet.sh
```

---

## Files Created

### Docker Scripts
- `build-docker.sh` - Builds Docker image from local codebase
- `docker-compose.yml` - Defines the 4-node network

### Common Scripts
- `setup-local-devnet.sh` - Generates keys and config files (used by both Docker and direct methods)
- `cleanup-devnet.sh` - Removes all containers and generated files

### Direct/Host Scripts
- `run-4-nodes-local.sh` - Starts 4 nodes directly on your machine
- `stop-local-nodes.sh` - Stops all locally running nodes

### Generated Files
- `config/` - Directory containing generated keys and validator registry (created by setup script)
  - `node0_key.hex` - Private key for node0
  - `node1_key.hex` - Private key for node1
  - `node2_key.hex` - Private key for node2
  - `node3_key.hex` - Private key for node3
  - `validator_registry.yml` - Maps node IDs to validator IDs

## How It Works

### Peer Discovery
1. Node0 starts first (defined in `depends_on` in docker-compose.yml)
2. Node0 generates a peer ID from its private key
3. Nodes 1-3 are configured with node0's multiaddr as a bootnode: `/ip4/172.20.0.10/udp/9000/quic-v1/p2p/16Uiu2HAmKMRmdX3M7Dc2bxVN4CQXAH9kQHw93ajiyGVqd63LY8kK`
4. When nodes 1-3 start, they connect directly to node0
5. Once connected, nodes can discover each other and form a mesh network

### Gossip Communication
- All nodes subscribe to the same gossipsub topics
- When a validator proposes a block, it broadcasts via gossipsub
- When a validator votes, it broadcasts via gossipsub
- All connected peers receive these messages
- The libp2p gossipsub protocol ensures efficient message propagation

### Validator Assignment
- Each node is assigned exactly one validator (see validator_registry.yml)
- node0 → validator 0
- node1 → validator 1
- node2 → validator 2
- node3 → validator 3
- Validators take turns proposing blocks in round-robin fashion
