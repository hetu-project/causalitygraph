# Hetu Causality Graph

A decentralized data collaboration network that achieves fast, verifiable causality and data consistency off-chain through verifiable logical clocks and a POCW consensus protocol.

## Definition

CausalityGraph is an off-chain decentralized "causality" data collaboration network. Through distributed relays (cRelay) and lightweight consensus (POCW), it enables:

- Recording client-submitted events and their causal order.
- Synchronizing and verifying causal order consistency across multiple nodes.
- Supporting the construction and querying of upper-layer multi-dimensional DependencyGraphs.

## Stack

Causality Graph is built on a robust architecture integrating several key components: **SDK, TEE relays, composite database, and graph dashboard**. Each component plays a crucial role in maintaining causal consistency of the decentralized data collaboration.  

<center><img src="graph_arch.png" alt="arch" width="900" height="650"></center>

### SDK ###

* Serves as the dApp interface for all interactions.
* EVM and Nostr compatible.

### TEE Relay ###
* Ensures secure and efficient data transmission.
* Employs POCW for data consistency and reliability.
* Each relay node is responsible for propagating data changes across the network while maintaining causal order.

### Composite Database ###

* The database forms a distributed graph-relation.
* Synchronizes data from Nostr relays, ensuring data consistency and integrity.

### Graph Dashboard ###

* Provides a visual interface for users to interact with the graph database.
* Allows exploration and manipulation of distributed documents.

## Protocol Components

### CausalityKey

CausalityKey is a protocol that leverages the event relationships stored in cRelay to enable automated and transparent value attribution and reward distribution. It ensures contributors are rewarded based on their verifiable participation in the network.

#### Structure

1. **Event Protocol (CIP: Causality Implementation Possibilities)**
   - Built on the Nostr protocol, defining custom kinds and tags with simple, extensible, and tamper-proof structures.
   - Extends through open protocols without relying on global consensus or specific P2P technologies.

2. **KeyToken Mapping Protocol**
   - Through client and cRelay interactions, cRelay nodes form a consensus on data statistics for specific events, known as CausalityKey.
   - Subkeys are defined within subnets, and incentive tokens are distributed through automated weighted calculations based on rules defined by the KeyToken Mapping Protocol.

### POCW (Proof of Causal Work)

- Establishes consensus on event stream ordering among relay nodes.
- Prevents single-point or minority node failures, ensuring consistent causal sequences across the network.

### cRelay

cRelay is a type of simple server designed to store client-submitted events that conform to protocol specifications. These relay servers can be hosted by anyone and can have any rules or internal policies they desire. Since the protocol is open, as long as any relay server is willing to host a specific server, clients can find their content on that relay server. To prevent relays from lying about data, the POCW algorithm is used to ensure the verifiability and consistency of off-chain data.

#### Features

- **Open Hosting**: Anyone can deploy and customize rules.
- **Scalability**: Supports multiple protocols as long as they follow CIP (Causality Implementation Possibilities).

#### Functions

- Receives client-submitted events encapsulated by CIP.
- Broadcasts new events to other cRelay nodes or subscribers.
- Stores a local event pool, supporting various internal policies (filtering, permissions, flow control).

### GraphSDK

- Provides a unified query interface:
  - Query causal paths by event ID or Key.
  - Perform topological sorting, critical path analysis, subgraph extraction, and influence calculation.

