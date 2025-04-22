# Causality Key with Nostr compatibility
## 1. Description

 This proposal integrates a Verifiable Logic Clock (VLC) into Nostr's event structure to enable decentralized message counting, identity and privilege management. User identities are bound to **ETH public key addresses** and events are signed using ETH signatures to ensure authenticity and non-repudiation. `tags` fields in Nostr events are used to encode VLC states and manage subspaces for VLC event statistics and fine-grained privilege control.

## 2. Key Features

- **Verifiable Logical Clock (VLC)**: Used to track event out-of-order in a distributed environment to ensure consistency.
- **ETH Public Key and Signature Compatibility**: Link user identities to ETH public keys and verify them with ETH signatures.
- **Subspace management**: Manage subspaces using custom event types ( `30100` for create, `30200` for join, `30300` for operate).
- **Flexible permission declarations**: Implement `auth` tags for operation-specific permissions, including operation type, dimension, and expiration time.

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
message subspaceKey {
  uint32 subspace_id = 1;    // Dimension 0: Subspace Identifier (32 bits)
  // Dimension 1-31: Subspace operation clock (31×32 bits)
  repeated uint32 clocks = 2 [packed=true]; 
}
```

- Dimension 1: Post
- Dimension 2: Propose
- Dimension 3: Vote
- Dimension 4: Invite
- Other dimensions can be extended as needed.
    - The following ModelGraph: model=5,data=6,compute=7,algo=8,valid=9
    - OR, auth is used to extend the definition of permissions

## 5. Generic Events

### 5.1 Subspace Creation Event (Kind 30100)

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
    ["subspace_name", "modelgraph"],
    ["ops", "post=1,propose=2,vote=3,invite=4,model=5,data=6,compute=7,algo=8,valid=9"],
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

`Generic execution operations`: 1: Post, 2: Propose, 3: Vote, 4: Invite

**1: Post (publish content)**: share knowledge, post updates

- Description: user posts content (e.g. announcements, documents) in the subspace.
- Message body:
    - ops: "post"
    - content_type: content type (e.g. "text", "markdown", "ipfs")
    - Optional: parent (reference to parent event hash)
- Example: Alice posts an announcement in subspace:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // Alice's ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30200,
  "tags": [
    // ["auth", "action=2", "dim=1", "exp=500000"],
    ["d", "subnet_op"],
    ["sid", "0xMG"],
    ["ops", "post"],
    ["content_type", "markdown"],
    ["parent", "parent-hash"]
  ],
  "content": "# Subspace Update\nWe have completed model optimization!",
  "sig": "<ETH signature>"
}
```

 **2: Propose**: push for subspace governance or parameter tuning

- DESCRIPTION: User proposes a subspace rule or operation that requires a subsequent vote.
- Message body:
    - ops: "propose"
    - proposal_id: proposal unique identifier
    - rules: proposed rules (e.g. "energy>2000")
- Example: Bob makes a proposal to raise the subspace join threshold:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // Bob's ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30200,
  "tags": [
    // ["auth", "action=2", "dim=2", "exp=500000"],
    ["d", "subnet_op"],
    ["sid", "0xMG"],
    ["ops", "propose"],
    ["proposal_id", "prop_001"],
    ["rules", "energy>2000"]
  ],
  "content": "Proposal to raise the energy requirement for joining the subspace to 2000",
  "sig": "<ETH signature>"
}
```

**3: Vote**: Enabling Decentralized Decision Making

- DESCRIPTION: Users vote on suggestions.
- Message body:
    - ops: "vote"
    - proposal_id: Identifier of the proposal that is the target of the vote
    - vote: vote value (e.g. "yes", "no")
- Example: Alice votes on Bob's proposal:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // Alice's ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30200,
  "tags": [
    ["auth", "action=2", "dim=3", "exp=500000"],
    ["d", "subnet_op"],
    ["sid", "0xMG"],
    ["ops", "vote"],
    ["proposal_id", "prop_001"],
    ["vote", "yes"],
  ],
  "content": "Agree to raise the energy requirement",
  "sig": "<ETH signature>"
}
```

**4: Invite**: extends subspace membership.

- DESCRIPTION: The user invites new members to join the subspace.
- Message body:
    - ops: "invite"
    - invitee_pubkey: ETH public key of the invitee
    - Optional: rules (join rules)
- Example: Alice invites Charlie to join the subspace:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // Alice's ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30200,
  "tags": [
    ["auth", "action=2", "dim=4", "exp=500000"],
    ["d", "subnet_op"],
    ["sid", "0xMG"],
    ["ops", "invite"],
    ["invitee_pubkey", "<Charlie’s ETH address>"],
    ["rules", "energy>1000"]
  ],
  "content": "Invite Charlie to join the Desci AI subspace",
  "sig": "<ETH signature>"
}
```

`Business Execution Operations`: 5: model, 6: data, 7: compute, 8: algo, 9: valid

**5: Model (upload model)**: (e.g., model operations):

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>", // ETH public key can be recovered from the signature
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30300,
  "tags": [
    ["auth", "action=3", "dim=4", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["ops", "model"],    // model=5, data=6, compute=7, algo=8, valid=9
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

 Alice performs a business model operation:

```json
{
  "id": "<32-bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32-bytes lowercase hex-encoded ETH address of the event creator>", // Alice ETH address
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30300,
  "tags": [
    ["auth", "action=3", "dim=4", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["ops", "model"],
    ["parent", "parent-hash"],
    ["contrib", "base:0.1", "data:0.6", "algo:0.3"]
  ],
  "content": "ipfs://bafy...",
  "sig": "<Alice ETH signature>"
}
```
