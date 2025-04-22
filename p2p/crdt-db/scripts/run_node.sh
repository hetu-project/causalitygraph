#!/bin/bash

# Build the OrbitDB example
go build -o orbitdb-example ./cmd/orbitdb-example

# Run the first node to create a new database
echo "Starting node 1 (creating new database)..."
./orbitdb-example -data ./data/node1 -listen "/ip4/0.0.0.0/tcp/4001" > node1.log 2>&1 &
NODE1_PID=$!

# Wait a bit for the first node to initialize
sleep 5

# Extract the database address from logs
DB_ADDRESS=$(grep "Database created with address:" node1.log | awk '{print $NF}')
if [ -z "$DB_ADDRESS" ]; then
    echo "Failed to get database address from node 1"
    kill $NODE1_PID
    exit 1
fi

echo "Database address: $DB_ADDRESS"

# Run the second node connecting to the first node's database
echo "Starting node 2 (connecting to existing database)..."
./orbitdb-example -data ./data/node2 -listen "/ip4/0.0.0.0/tcp/4002" -db "$DB_ADDRESS" > node2.log 2>&1 &
NODE2_PID=$!

# Run the third node connecting to the first node's database
echo "Starting node 3 (connecting to existing database)..."
./orbitdb-example -data ./data/node3 -listen "/ip4/0.0.0.0/tcp/4003" -db "$DB_ADDRESS" > node3.log 2>&1 &
NODE3_PID=$!

echo "All nodes are running. Press Ctrl+C to stop."
echo "Check node1.log, node2.log, and node3.log for outputs."

# Wait for Ctrl+C
trap "kill $NODE1_PID $NODE2_PID $NODE3_PID; echo 'All nodes stopped.'; exit 0" INT
wait