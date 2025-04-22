# OrbitDB Go Example

This is a Go implementation of the OrbitDB example that mimics the functionality of the [JS version](https://github.com/alexeyvolkoff/orbitdb-example).

## Prerequisites

- Go 1.16 or higher
- A running IPFS daemon

## Setup

1. Start an IPFS daemon:

```bash
ipfs daemon
```

2. Build the example:

```bash
go build -o orbitdb-example ./cmd/orbitdb-example
```

## Usage

### Running a single node

```bash
./orbitdb-example -data ./data/mynode
```

### Command line options

- `-data`: Data directory path (default: "./data")
- `-db`: OrbitDB address to connect to (if connecting to an existing database)
- `-listen`: Libp2p listen address (default: "/ip4/0.0.0.0/tcp/4001")
- `-ipfs`: IPFS API endpoint (default: "localhost:5001")

### Running multiple nodes

Use the provided script to run three nodes that will automatically connect:

```bash
chmod +x scripts/run_nodes.sh
./scripts/run_nodes.sh
```

This will:
1. Start the first node that creates a new database
2. Extract the database address from the logs
3. Start two more nodes that connect to the same database
4. Output logs to node1.log, node2.log, and node3.log

### Manual multi-node setup

1. Start the first node to create a new database:

```bash
./orbitdb-example -data ./data/node1 -listen "/ip4/0.0.0.0/tcp/4001"
```

2. Note the database address from the logs (e.g., `/orbitdb/QmYourCID/onmydisk`)

3. Start additional nodes connecting to the same database:

```bash
./orbitdb-example -data ./data/node2 -listen "/ip4/0.0.0.0/tcp/4002" -db "/orbitdb/QmYourCID/onmydisk"
```

## How it works

1. The application creates or loads a peer identity
2. It connects to a local IPFS node
3. It sets up a libp2p host
4. It creates or connects to an OrbitDB database
5. When connected, it adds a random text to IPFS and stores the CID in OrbitDB
6. It listens for updates to the database and fetches files from IPFS when new entries are added
7. On shutdown, it prints the final state of the database

## License

MIT