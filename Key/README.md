# Hetu Causality Key Protocol

The Hetu Causality Key Protocol is a foundational component of the Hetu Causality Graph, enabling decentralized and verifiable causality management. It integrates Verifiable Logical Clocks (VLC) and token protocols to support event tracking, identity management, and value attribution in distributed systems.

## Key Features

### 1. Verifiable Logical Clock (VLC)
- Tracks event causality and ensures consistency in distributed environments.
- Encodes VLC states in Nostr event `tags` for subspace management and fine-grained privilege control.

### 2. CausalityKey
- Automates and transparently attributes value and rewards based on event relationships stored in cRelay.
- Supports subspace creation, operations, and governance through defined event kinds.

### 3. Key Token Protocol
- Extends the functionality of CausalityKey by enabling token operations such as issuance, transfer, approval, and minting.
- Provides a standardized framework for tokenized value distribution.

## Protocol Components

### Event Protocol (CIP: Causality Implementation Possibilities)
- Built on the Nostr protocol, defining custom kinds and tags with extensible and tamper-proof structures.
- Supports open protocol extensions without reliance on global consensus or specific P2P technologies.

### KeyToken Mapping Protocol
- Facilitates automated weighted calculations and token distribution based on predefined rules.
- Enables subkey definitions within subnets for localized value attribution.

## Event Kinds

Refer to the [CausalityKeyList.md](./CausalityKeyList.md) for a comprehensive list of event kinds, including governance and token operations.

### Token Protocol Standard

The [Key_Token_Protocol_Standard.md](./Key_Token_Protocol_Standard.md) outlines the implementation of token operations, including:
- **Issue Token (Kind 30320):** Create and configure new tokens.
- **Transfer (Kind 30321):** Facilitate token transfers between accounts.
- **Approve (Kind 30322):** Authorize accounts to spend tokens on behalf of others.
- **Mint (Kind 30323):** Define rules for creating new tokens.

## Use Cases

### Subspace Management
- Create and manage subspaces with defined operations and rules.
- Enable governance through proposals, voting, and invitations.

### Tokenized Incentives
- Reward contributors based on verifiable participation.
- Distribute tokens using automated and transparent mechanisms.

For more details, refer to the [CIP01 CausalityKey.md](./CausalityKey.md) document.
