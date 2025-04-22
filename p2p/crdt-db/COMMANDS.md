# OrbitDB Go Example Commands

## Building the Application

```bash
go build -o orbitdb-example ./cmd/orbitdb-example
```

## Running Nodes Manually

### First Node (Create new database)

```bash
./orbitdb-example -data ./data/node1 -listen "/ip4/0.0.0.0/tcp/4001"
```

Look for the line in the output that says `Database created with address: /orbitdb/[CID]/onmydisk`
Copy this address for the next steps.

### Second Node (Connect to existing database)

```bash
./orbitdb-example -data ./data/node2 -listen "/ip4/0.0.0.0/tcp/4002" -db "/orbitdb/[CID]/onmydisk"
```

### Third Node (Connect to existing database)

```bash
./orbitdb-example -data ./data/node3 -listen "/ip4/0.0.0.0/tcp/4003" -db "/orbitdb/[CID]/onmydisk"
```

## Using the Automatic Script

```bash
chmod +x scripts/run_nodes.sh
./scripts/run_nodes.sh
```

## Configuration Options

- `-data`: Data directory path (default: "./data")
- `-db`: OrbitDB address to connect to (if connecting to an existing database)
- `-listen`: Libp2p listen address (default: "/ip4/0.0.0.0/tcp/4001")
- `-ipfs`: IPFS API endpoint (default: "localhost:5001")

## Example with Custom IPFS API Endpoint

```bash
./orbitdb-example -data ./data/node1 -ipfs "192.168.1.100:5001"
```

## Running in Docker

Build the Docker image:

```bash
docker build -t orbitdb-example .
```

Run the container:

```bash
docker run -p 4001:4001 -v $(pwd)/data:/app/data orbitdb-example
```

Or connect to an existing database:

```bash
docker run -p 4002:4001 -v $(pwd)/data:/app/data orbitdb-example -db "/orbitdb/[CID]/onmydisk"
```