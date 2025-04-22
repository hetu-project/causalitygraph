# P2P Project Overview

The `p2p` folder contains multiple subprojects and tools focused on the Nostr protocol and distributed systems development. It provides extensive functionality, including event signing, key generation, message publishing, logical clock implementation, and more.

## Project Structure

- **`crdt-db/`**: Contains documentation and implementation related to CRDT databases.
- **`go-nostr-sdk/`**: A Go-based Nostr SDK offering connection options, event handling, benchmarking, and more.
- **`js-nostr-sdk/`**: A JavaScript-based Nostr SDK supporting key generation, event signing, NIP encoding/decoding, and more.
- **`nak-nostr/`**: A powerful Nostr toolkit supporting event signing, key generation, event querying, NIP encoding/decoding, and more.
- **`zeb/`**: A P2P relay network supporting verifiable VLC (Virtual Logical Clock) causal ordering, currently in prototype stage.
- **`znostr/`**: A Nostr client CLI and related documentation, supporting user creation, message publishing, private messaging, and more.

## Features Overview

### `go-nostr-sdk`
- Provides core functionality for the Nostr protocol.
- Supports multiple connection options (e.g., WebSocket).
- Includes performance benchmarks and event handling logic.

### `js-nostr-sdk`
- A toolkit for JavaScript developers.
- Supports key generation, public/private key conversion, event signing, and verification.
- Implements NIP-19, NIP-27, and other protocol extensions.

### `nak-nostr`
- A command-line tool with the following features:
  - Generate key pairs.
  - Sign and publish events.
  - Query and decode NIP encodings.
  - Mount Nostr data using a FUSE filesystem.

### `zeb`
- Implements a P2P relay network with VLC support.
- Provides efficient LMDB storage and causal ordering for event processing.

### `znostr`
- A feature-rich CLI client.
- Supports user management (create, delete users).
- Enables message publishing, private messaging, and channel messaging.
- Offers detailed help documentation and usage guides.

## Installation and Usage

### Installation
- Refer to the `README.md` file in each subproject for detailed installation steps.
- For example, the `znostr` project can be installed as follows:
  1. Install the Rust compiler and Cargo.
  2. Clone the project repository and run `cargo build --release`.

### Examples
#### Publish an Event Using `nak-nostr`
```shell
~> nak event --sec 01 -c 'Hello Nostr!' --tag t=example relay.damus.io
