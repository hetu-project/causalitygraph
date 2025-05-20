# Causality Key Event
## 1. Description

 This proposal integrates a Verifiable Logic Clock (VLC) into Nostr's event structure to enable decentralized message counting, identity and privilege management. User identities are bound to **ETH public key addresses** and events are signed using ETH signatures to ensure authenticity and non-repudiation. `tags` fields in Nostr events are used to encode VLC states and manage subspaces for VLC event statistics and fine-grained privilege control.

## 2. Key Features

- **Verifiable Logical Clock (VLC)**: Used to track event out-of-order in a distributed environment to ensure consistency.
- **ETH Public Key and Signature Compatibility**: Link user identities to ETH public keys and verify them with ETH signatures.
- **Subspace management**: Manage subspaces using custom event types ( `30100` for create, `30200` for join, `30300` and above for operations).
- **Flexible permission declarations**: Implement `auth` tags for operation-specific permissions, including operation type, causality key, and expiration time.

---

## 3. Nostr Event Structure

 The underlying Nostr event structure is consistent with that defined in NIP-01:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": "<integer between 0 and 65535>",
  "tags": [
    ["<arbitrary string>", "..."],
    // ...
  ],
  "content": "<arbitrary string>",
  "sig": "<64 bytes lowercase hex-encoded ETH signature, with the content being the sha256 hash of the serialized event data, which is the same as the 'id' field>"
}
```

## 4. subspaceKey structure

`The subspaceKey` defines the subspace identifier and operation clock, encoded in `tags`:

```bash
message subspace {
  uint32 subspace_id = 1;    // Dimension 0: Subspace Identifier (32 bits)
  // Subspace operation clock
  repeated causalityKey keys = 2 [packed=true]; 
}

message causalityKey {
	uint32 key = 1;       // causality key identifier
	unt64  counter = 1;   // Lamport clock
}
```

For example, 

- Dimension 1: Post, causality key 30300
- Dimension 2: Propose, causality key 30301
- Dimension 3: Vote, causality key 30302
- Dimension 4: Invite, causality key 30303
- Other dimensions can be extended as needed.
    - The following ModelGraph:
        - model=30304,data=30305,compute=30306,algo=30307,valid=30308
    - OR, auth is used to extend the definition of permissions

## 5. Generic Events

### 5.1 Subspace Creation Event (Kind 30100)

Creating a subspace is equivalent to creating a set of causality keys.

- Message body field description:
    - d: "subspace_create", d stands for define, used to define generic time type
    - sid: "0xMG", sid refers to the unique hash index of subspace_Id.
        - sid = hash(subspace_name + ops + rules)
    - subspace_name: name of the subspace
    - ops: define all actions in the subspace
    - rules (optional): rules for defining joins, or other custom rules
- Example: Creating a new subspace:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // ETH public key can be recovered from the signature
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30100,
  "tags": [
    ["d", "subspace_create"],
    ["sid", "0xMG"],
    ["subspace_name", "governance"],
    ["category","web3"],
    ["ops", "post=30300,propose=303001,vote=30302,invite=30303"],
    ["rules", "energy>1000"]
  ],
  "content": "{\"desc\":\"Desci AI model collaboration subspace\", \"img_url\": \"http://imge_addr.png\"}",
  "sig": "<ETH signature>"
}
```

### 5.2 Subspace Join Event (Kind 30200)

 Allows the user to join an existing subspace:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // ETH public key can be recovered from the signature
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30200,
  "tags": [
    ["d", "subspace_join"],
    ["sid", "0xMG"],
    ["rules", "energy>1000"],         
  ],
  "content": "*12345",
  "sig": "<ETH signature>"
}
```

### 5.3 Subspace Operation Events (Kind 30300)

To facilitate protocol parsing, each operation (Post, Propose, Vote, Invite, Model, Data, Compute, Algo, Valid) is assigned a unique **Kind value**. The operation-specific information is defined in the tags field. 

**5.3.1 Subspace Operation Events**

The operations are listed in the following table:

- First subspace is `governance subspace`，it contains a set of causality keys:
    - 30300: Post, 30301: Propose, 30302: Vote, 30303: Invite

| Kind Value | Event Name | Purpose | Key Tags Structure |
| --- | --- | --- | --- |
| 30300 | Post | Publish content (e.g., announcements, documents) in the subspace | ["auth", "d":"subspace_op", "sid", "content_type", "parent"] |
| 30301 | Propose | Propose subspace rules or operations, requiring subsequent voting | ["auth", "d":"subspace_op", "sid", "proposal_id", "rules"] |
| 30302 | Vote | Vote on proposals for decentralized decision-making | ["auth", "d":"subspace_op", "sid", "proposal_id", "vote"] |
| 30303 | Invite | Invite new members to join the subspace | ["auth", "d":"subspace_op", "sid", "inviter_addr", "rules"] |
| 30304	| mint	| mint credit token, and issue to membership in community	| ["auth", "d":"subspace_op", "sid", "token_name", "token_symbol",”token_decimals”,”initial_supply”] |

**5.3.2 Generic Operation Event Structure**

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", 
  "created_at": "<Unix timestamp in seconds>",
  "kind": "<30300-30308>",
  "tags": [
    ["auth", "action=<mask>", "key=<key-Id>", "exp=<expiration clock>"],
    ["d", "subspace_op"],
    ["sid", "<subspace ID>"],
    ["parent", "parent-hash"]
    // Operation-specific tags
  ],
  "content": "<opration content>",
  "sig": "<ETH signature>"
}
```

- auth tag: Defines permissions with action (mask: 1=read, 2=write, 4=execute), key (causality key ID), and exp (expiration clock value).

**5.3.3 Case examples**

**1: Post (publish content)**: share knowledge, post updates

- Description: user posts content (e.g. announcements, documents) in the subspace.
- Message body:
    - op: "post"
    - content_type: content type (e.g. "text", "markdown", "ipfs")
    - parent: parent event hash set (reference to parent event hash)
- Example: Alice posts an announcement in subspace:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // Alice's ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30300,
  "tags": [
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "post"],
    ["content_type", "markdown"],
    ["parent", "parent-hash", "..."]
  ],
  "content": "# Subspace Update\nWe have completed model optimization!",
  "sig": "<ETH signature>"
}
```

 **2: Propose**: push for subspace governance or parameter tuning

- DESCRIPTION: User proposes a subspace rule or operation that requires a subsequent vote.
- Message body:
    - op: "propose"
    - proposal_id: proposal unique identifier
    - parent: parent event hash set (reference to parent event hash)
    - rules: proposed rules (e.g. "energy>2000")
- Example: Bob makes a proposal to raise the subspace join threshold:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // Bob's ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30301,
  "tags": [
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "propose"],
    ["proposal_id", "prop_001"],
    ["parent", "parent-hash", "..."]
    ["rules", "energy>2000"]
  ],
  "content": "Proposal to raise the energy requirement for joining the subspace to 2000",
  "sig": "<ETH signature>"
}
```

**3: Vote**: Enabling Decentralized Decision Making

- DESCRIPTION: Users vote on suggestions.
- Message body:
    - op: "vote"
    - proposal_id: Identifier of the proposal that is the target of the vote
    - parent: parent event hash set (reference to parent event hash)
    - vote: vote value (e.g. "yes", "no")
- Example: Alice votes on Bob's proposal:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // Alice's ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30302,
  "tags": [
    ["auth", "action=2", "dim=3", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "vote"],
    ["proposal_id", "prop_001"],
    ["parent", "parent-hash", "..."]
    ["vote", "yes"],
  ],
  "content": "Agree to raise the energy requirement",
  "sig": "<ETH signature>"
}
```

**4: Invite**: extends subspace membership.

- DESCRIPTION: The user invites new members to join the subspace.
- Message body:
    - op: "invite"
    - inviter_addr: ETH public key of the inviter
    - parent: parent event hash set (reference to parent event hash)
    - Optional: rules (join rules)
- Example: Alice invites Charlie to join the subspace:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // Charlie's ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30303,
  "tags": [
    ["auth", "action=2", "dim=4", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "invite"],
    ["inviter_addr", "<Alice’s ETH address>"],
    ["parent", "parent-hash", "..."]
    ["rules", "energy>1000"]
  ],
  "content": "Invite Charlie to join the Desci AI subspace",
  "sig": "<ETH signature>"
}
```
**5: Mint (mint erc20 & airdrop)**

- Description: mint credit token, and issue to membership in community
- Message body:
    - op: "mint"
    - parent: parent (reference to parent event hash)
    - token related:
    - token_name: token name Token name
    - token_synbol: token symbol Token symbol
    - token_decimals:  token decimals Token decimals
    - initial_supply: initialSupply Initial amount to mint (in token units, not wei)
    - drop_ratio: Each causality key proportion when token distribution
- Example: Alice mint credit token in subspace:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // Alice's ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30304,
  "tags": [
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "mint"],
    ["parent", "parent-hash"]
    ["token_name", "sub-name"],
    ["token_symbol", "SYM"],
    ["token_decimals", "18"],
    ["initial_supply", "100"],
    ["drop_ratio", "30300:2,30301:2,30302:1,30303:3,30304:10"],
  ],
  "content": "",
  "sig": "<ETH signature>"
}
```

**5.3.4 Define a subspace (eg: modelgraph)**

`Business Execution Operations`: 5: model, 6: data, 7: compute, 8: algo, 9: valid

| Kind Value | Event Name | Purpose | Key Tags Structure |
| --- | --- | --- | --- |
| 30404 | Model | Submit a new model version | ["auth", "d":"subspace_op", "sid", "parent", "contrib"] |
| 30405 | Data | Submit training datasets | ["auth", "d":"subspace_op", "sid", "size"] |
| 30406 | Compute | Submit computational tasks | ["auth", "d":"subspace_op", "sid", "compute_type"] |
| 30407 | Algo | Submit algorithm code or updates | ["auth", "d":"subspace_op", "sid", "algo_type"] |
| 30408 | Valid | Submit validation task results | ["auth", "d":"subspace_op", "sid", "valid_result"] |

**5.3.4.1 Create a** modelgraph

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // ETH public key can be recovered from the signature
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30100,
  "tags": [
    ["d", "subspace_create"],
    ["sid", "0xMG"],
    ["subspace_name", "modelgraph"],
    ["ops", "post=30300,propose=303001,vote=30302,invite=30303,model=30304,data=30305,compute=30306,algo=30307,valid=30308"],
    ["rules", "energy>1000"]
  ],
  "content": "{\"desc\":\"Desci AI model collaboration subspace\", \"img_url\": \"http://imge_addr.png\"}",
  "sig": "<ETH signature>"
}
```

**5.3.4.2 Model (upload model)**: (e.g., model operations):

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // ETH public key can be recovered from the signature
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30304,
  "tags": [
    ["auth", "action=3", "dim=4", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "model"],
    ["parent", "parent-hash"], // parent event hash
    ["contrib", "base:0.1", "data:0.6", "algo:0.3"],
  ],
  "content": "ipfs://bafy...",
  "sig": "<ETH signature>"
}
```

- `auth` tag: defines permissions, including `action` (mask: 1=read, 2=write, 4=execute), `dim` (dimension) and `exp` (expired clock value).

---

## 6. Examples

### 6.1 Subspace creation

 Alice creates a "Desci AI Model Collaboration Subspace":

```json
{
  "id": "<32-bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32-bytes lowercase hex-encoded ETH address of the event creator>", // Alice ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30100,
  "tags": [
    ["d", "subspace_create"],
    ["sid", "0xMG"],
    ["subspace_name", "modelgraph"],
    ["ops", "model=5,data=6,compute=7,algo=8,valid=9"],
    ["rules", "energy>1000"]
  ],
  "content": "{\\"desc\\":\\"Desci AI collaborative subspace for models\\"}",
  "sig": "<Alice ETH signature>"
}
```

### 6.2 Subspace Joining

 Bob joins Alice's subspace:

```json
{
  "id": "<32-bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32-bytes lowercase hex-encoded ETH address of the event creator>", // Bob ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30200,
  "tags": [
    ["d", "subspace_join"],
    ["sid", "0xMG"]
  ],
  "content": "*12345",
  "sig": "<Bob ETH signature>"
}
```

### 6.3 Subspace Operations

 For `generic execution operations`, see the Post, Propose, Vote, and Invite operation examples above.

---
