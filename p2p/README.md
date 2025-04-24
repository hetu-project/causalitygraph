# P2P Project Overview

The `p2p` folder contains multiple subprojects and tools focused on the [Causality Key](./CausalityKeyWithNostr.md) with Nostr protocol and distributed systems development. It provides extensive functionality, including causality subspace lifecycle managing, general action configuration templates, causality graph generation, logical clock implementation, Ethereum [EIP-191]((https://eips.ethereum.org/EIPS/eip-191)) signing, key generation, message publishing, and more.

## Project Structure

- **`cRalay/`**: A powerful Nostr relayer with causality-graph supporting event distributed storage, event causality ordering by [VLC (Virtual Logical Clock)](https://github.com/hetu-project/chronos.git), event querying, configurable business subspace management
, and more.
- **`crdt-db/`**: Contains documentation and implementation related to CRDT databases.
- **`go-sdk/`**: A Go-based SDK offering Ethereum EIP-191 signing, event templates, event handling, benchmarking, and more.
- **`js-sdk/`**: A JavaScript-based SDK supporting subspace lifecycle managing, eth event signing, common encoding/decoding, and more.
- **`zeb/`**: A P2P relay network supporting verifiable VLC (Virtual Logical Clock) causal ordering, currently in prototype stage.
- **`znostr/`**: The znostr's initial idea was to integrate the ability of logical clocks into social networks, and developers can easily utilize it in their familiar development environment and applications.

## Features Overview

### `cRalay`
- Verifiable Logical Clock and Causality Graph
    - Implements a distributed logical clock for event ordering
    - Supports causality graph generation for event relationships
    - Track causality between distributed events
    - Maintain consistent ordering across nodes
    - Detect concurrent operations
    - Enable distributed consensus
- Nostr & CRDT-DB Integration
    - Decentralized communication protocol
    - Publish and subscribe to events securely
    - Relay-based message broadcasting
    - Event distributed storage and synchronization
- Ethereum EIP-191 Signature
    - Secure message signing using Ethereum wallets
    - Identity verification via cryptographic proofs
    - EIP-191 ensures structured, tamper-proof signatures
    - Compatible with Ethereum ecosystem tools

### `go-sdk`
- Provides core causality relation and causality graph functionality for the Nostr protocol.
- Supports Ethereum EIP-191 signing for secure message signing and identity verification.
- Offers event templates for standardized event creation.
- Includes tools for event handling and processing.
- Provides benchmarking utilities for performance evaluation.

### `js-sdk`
- A JavaScript-based SDK for managing business subspace lifecycles.
- Supports Ethereum EIP-191 signing for secure and tamper-proof event signatures.
- Provides tools for encoding and decoding common formats.
- Includes event templates for standardized event creation.
- Enables key generation and public/private key conversion.
- Implements [Causality Key](./CausalityKeyWithNostr.md) protocol extensions for extending nostr 's capacity.

### `zeb`
- Implements a P2P relay network with VLC support.
- Provides efficient DHT storage and causal ordering for event processing.

### `znostr`
- Znostr is a network protocol framework.
- Supports common management of nostr relayer (create,publish,query).
- Enables message publishing, private messaging, and channel messaging.
- Offers detailed [help documentation and usage guides](./znostr/docs/SUMMARY.md).

## Installation and Usage

### Installation
- Refer to the `README.md` file in each subproject for detailed installation steps.
- For example, the `znostr` project can be installed as follows:
  1. Install the Rust compiler and Cargo.
  2. Clone the project repository and run `cargo build --release`.