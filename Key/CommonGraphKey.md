# Common Graph Key Events
## 1. Description

This proposal extends the Causality Key protocol to generalizes key collaborative primitives (Project, Task, Graph) for use across all subspaces. 
It introduces a set of event types that can be used in any subspace, allowing for flexible and reusable event structures. The events are designed to be extensible and composable, enabling users to create complex workflows and interactions within the Causality Graph framework.

## 1.1 Key Features

- **ETH Public Key and Signature Compatibility**: Link user identities to ETH public keys and verify them with ETH signatures.
- **Flexible permission declarations**: Implement `auth` tags for operation-specific permissions, including operation type, key ID, and expiration time.
- **Cross-subspace reusability**: Any event type defined here can be used across different subspaces.
- **Extensible structure**: The event structure can be extended while maintaining compatibility.

## 2. General Events (Reusable Across Subspaces)

| Kind Value | Event Name   | Purpose                                      | Key Tags Structure |
|------------|-------------|----------------------------------------------|--------------------|
| 30101      | Project     | Define a project within a subspace           | ["auth", "d":"subspace_op", "op":"project", "sid", "project_id", "name", "desc", "members", "status"] |
| 30102      | Task        | Task belonging to a project                   | ["auth", "d":"subspace_op", "op":"task", "sid", "project_id", "task_id", "title", "assignee", "status", "deadline", "priority", ...] |
| 30103      | Entity      | Define an entity node in a knowledge graph    | ["auth", "d":"subspace_op", "op":"entity", "sid", "entity_name", "entity_type"] |
| 30104      | Relation    | Define a relation between two entities        | ["auth", "d":"subspace_op", "op":"relation", "sid", "from", "to", "relation_type", "context", "weight", "description"] |
| 30105      | Observation | Attach an observation to an entity            | ["auth", "d":"subspace_op", "op":"observation", "sid", "entity_name", "observation"] |

### 2.1 Project Event (Kind 30101)
```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30101,
  "tags": [
    ["auth", "action=3", "key=30101", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "project"],
    ["sid", "0xOR"],
    ["project_id", "proj_001"],
    ["name", "Quantum NLP"],
    ["desc", "Research on quantum natural language processing"],
    ["members", "0xAlice,0xBob"],
    ["status", "active"]
  ],
  "content": "",
  "sig": "<ETH signature>"
}
```

### 2.2 Task Event (Kind 30102)
```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30102,
  "tags": [
    ["auth", "action=3", "key=30102", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "task"],
    ["sid", "0xOR"],
    ["project_id", "proj_001"],
    ["task_id", "task_001"],
    ["title", "Literature Review"],
    ["assignee", "0xBob"],
    ["status", "in_progress"],
    ["deadline", "1712345678"],
    ["priority", "high"]
  ],
  "content": "Review recent papers on quantum NLP.",
  "sig": "<ETH signature>"
}
```

### 2.3 Entity Event (Kind 30103)
```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30103,
  "tags": [
    ["auth", "action=3", "key=30103", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "entity"],
    ["sid", "0xOR"],
    ["entity_name", "John_Smith"],
    ["entity_type", "person"]
  ],
  "content": "{\"observations\":[\"Speaks fluent Spanish\"]}",
  "sig": "<ETH signature>"
}
```

### 2.4 Relation Event (Kind 30104)
```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30104,
  "tags": [
    ["auth", "action=3", "key=30104", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "relation"],
    ["sid", "0xOR"],
    ["from", "John_Smith"],
    ["to", "Anthropic"],
    ["relation_type", "works_at"],
    ["weight", "0.85"],
    ["description", "{}"]
  ],
  "content": "",
  "sig": "<ETH signature>"
}
```

### 2.5 Observation Event (Kind 30105)
```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30105,
  "tags": [
    ["auth", "action=3", "key=30105", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "observation"],
    ["sid", "0xOR"],
    ["entity_name", "John_Smith"],
    ["observation", "Graduated in 2019"]
  ],
  "content": "",
  "sig": "<ETH signature>"
}
```

## 3. Example Event Flows

### 3.1 Create a Project and Task

```json
// 1. Create a project
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30101,
  "tags": [
    ["auth", "action=3", "key=30101", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "project"],
    ["sid", "0xOR"],
    ["project_id", "proj_001"],
    ["name", "Quantum NLP"],
    ["desc", "Research on quantum natural language processing"],
    ["members", "0xAlice,0xBob"],
    ["status", "active"]
  ],
  "content": "",
  "sig": "<ETH signature>"
}

// 2. Create a task in the project
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30102,
  "tags": [
    ["auth", "action=3", "key=30102", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "task"],
    ["sid", "0xOR"],
    ["project_id", "proj_001"],
    ["task_id", "task_001"],
    ["title", "Literature Review"],
    ["assignee", "0xBob"],
    ["status", "in_progress"],
    ["deadline", "1712345678"],
    ["priority", "high"]
  ],
  "content": "Review recent papers on quantum NLP.",
  "sig": "<ETH signature>"
}
```

### 3.2 Knowledge Graph Operations

```json
// 1. Create an entity
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30103,
  "tags": [
    ["auth", "action=3", "key=30103", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "entity"],
    ["sid", "0xOR"],
    ["entity_name", "John_Smith"],
    ["entity_type", "person"]
  ],
  "content": "{\"observations\":[\"Speaks fluent Spanish\"]}",
  "sig": "<ETH signature>"
}

// 2. Add a relation
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30104,
  "tags": [
    ["auth", "action=3", "key=30104", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "relation"],
    ["sid", "0xOR"],
    ["from", "John_Smith"],
    ["to", "Anthropic"],
    ["relation_type", "works_at"],
    ["weight", "0.85"],
    ["description", "{}"]
  ],
  "content": "",
  "sig": "<ETH signature>"
}

// 3. Add an observation
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30105,
  "tags": [
    ["auth", "action=3", "key=30105", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "observation"],
    ["sid", "0xOR"],
    ["entity_name", "John_Smith"],
    ["observation", "Graduated in 2019"]
  ],
  "content": "",
  "sig": "<ETH signature>"
}
```