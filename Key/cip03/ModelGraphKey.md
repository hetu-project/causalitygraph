# ModelGraph Causality Key Events

## 1. Description

This proposal extends the Causality Key protocol to support the ModelGraph platform, which enables users to create and manage fine-tuned large models and corresponding DAOs without coding skills. By leveraging the Verifiable Logic Clock (VLC) within the Nostr event structure, ModelGraph facilitates collaborative model upgrades and rapid issuance of governance tokens. User identities are bound to **ETH public key addresses**, and all model-related events are signed using ETH signatures to ensure authenticity and provenance tracking in model development.

To enhance functionality, this extension introduces new Causality Key events for AI conversations and session management, allowing the platform to record user-AI interactions and manage conversation lifecycles effectively. Additionally, the **Conversation Event** now includes an `interaction_hash` field to uniquely identify each interaction and support linking of interaction sequences.

## 2. Key Features

- **No-Code Model Fine-tuning**: Create and manage fine-tuned models through a user-friendly interface.
- **Collaborative Model Development**: Enable team collaboration on model development and upgrades.
- **DAO Integration**: Seamlessly integrate with DAO governance for model management.
- **Dataset Management**: Track and version control training datasets with proper attribution.
- **Tokenized Incentives**: Reward contributors through governance tokens.
- **Model Provenance**: Track model versions, training data, and contributors.
- **AI Conversation Management**: Facilitate and record interactions between users and AI models.
- **Session Management**: Manage the lifecycle of conversation sessions, including creation, updates, and termination.
- **Interaction Hashing**: Each conversation event includes a unique `interaction_hash` for identification and sequence linking.

---

## 3. ModelGraph Subspace

The ModelGraph platform extends the basic Causality Key system with new kinds of events specific to model development workflows, now including AI conversations and session management.

### 3.1 Subspace Creation Event (Kind 30100)

The subspace creation event is updated to include new operation types for conversation and session management:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30100,
  "tags": [
    ["d", "subspace_create"],
    ["sid", "0xMG"],
    ["subspace_name", "modelgraph"],
    ["category", "ai"],
    ["ops", "project=30101,task=30102,post=30300,propose=30301,vote=30302,invite=30303,mint=30304,dataset=30405,finetune=30409,conversation=30410,session=30411"],
    ["rules", "energy>500"]
  ],
  "content": "{\"desc\":\"Collaborative space for model development, fine-tuning, and AI interactions\", \"img_url\": \"http://image_addr.png\"}",
  "sig": "<ETH signature>"
}
```

### 3.2 ModelGraph Specific Event Types

The event types table is expanded to include new events for AI conversation and session management, with the addition of `interaction_hash` in the Conversation Event:

| Kind Value | Event Name      | Purpose                                         | Key Tags Structure                                                                 |
|------------|-----------------|-------------------------------------------------|------------------------------------------------------------------------------------|
| 30405      | Dataset         | Submit or manage training datasets             | ["auth", "d":"subspace_op", "op":"dataset", "sid", "project_id", "task_id", "category", "format", "contributors"] |
| 30409      | Finetune        | Create or update fine-tuning experiments       | ["auth", "d":"subspace_op", "op":"finetune", "sid", "project_id", "task_id", "dataset_id", "provider_id"] |
| 30410      | Conversation    | Record interactions between users and AI models| ["auth", "d":"subspace_op", "op":"conversation", "sid", "session_id", "user_id", "model_id", "timestamp", "interaction_hash"] |
| 30411      | Session         | Manage the lifecycle of conversation sessions  | ["auth", "d":"subspace_op", "op":"session", "sid", "session_id", "action", "user_id", "start_time", "end_time"] |

Other causality key definitions refer to [CausalityKeyList](./CausalityKeyList.md).

---

## 4. Operation Event Details

### 4.1 Dataset Event (Kind 30405)

Used to submit, manage, or update training datasets for model development. The dataset can contain both manually curated data and AI-generated samples.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30405,
  "tags": [
    ["auth", "action=3", "key=30405", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "dataset"],
    ["project_id", "<project_id>"],
    ["task_id", "<task_id>"],
    ["category", "Sales"],
    ["format", "jsonl"],
    ["contributors", "contributor_hash_fir,contributor_hash_sec"]
  ],
  "content": {
    "dataset": [
      {
        "messages": [
          {"role": "system", "content": "You are a customer service assistant"},
          {"role": "user", "content": "How to change delivery address?"},
          {"role": "assistant", "content": "You can follow these steps:\n1. Go to order details\n2. Click 'Change Address'\n3. Enter new address information\n4. Save changes"}
        ]
      },
      {
        "messages": [
          {"role": "system", "content": "You are a customer service assistant"},
          {"role": "user", "content": "How to specify delivery time?"},
          {"role": "assistant", "content": "You can follow these steps:\n1. Go to order details\n2. Click 'Change Delivery Time'\n3. Enter new delivery time\n4. Save changes"}
        ]
      }
    ],
    "generate": {
      "topic": ["AI Applications"],
      "num_samples": 3,
      "model_name": "Qwen/Qwen2.5-72B-Instruct-Turbo",
      "provider": "together_ai",
      "human_guidance": "Generate samples about AI applications in healthcare"
    }
  },
  "sig": "<ETH signature>"
}
```

### 4.2 Finetune Event (Kind 30409)

Used to create and manage fine-tuning experiments for models.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30409,
  "tags": [
    ["auth", "action=3", "key=30409", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xFT"],
    ["op", "finetune"],
    ["project_id", "<project_id>"],
    ["task_id", "<task_id>"],
    ["dataset_id", "<dataset_id>"],
    ["provider_id", "<provider_id>"],
    ["model_name", "Qwen/Qwen2.5-72B-Instruct-Turbo"]
  ],
  "content": "{\"name\":\"Fine-tuning Experiment 1\",\"description\":\"This is a test fine-tuning experiment\",\"hyperparameters\":{\"learning_rate\":0.001,\"epochs\":3}}",
  "sig": "<ETH signature>"
}
```

### 4.3 Conversation Event (Kind 30410)

Used to record interactions between users and AI models, capturing the dialogue history and context. The event now includes an `interaction_hash` field to uniquely identify the current interaction.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30410,
  "tags": [
    ["auth", "action=3", "key=30410", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "conversation"],
    ["session_id", "<session_id>"],
    ["user_id", "<user_id>"],
    ["model_id", "<model_id>"],
    ["timestamp", "<interaction_timestamp>"],
    ["interaction_hash", "<current_interaction_hash>"]  // New field
  ],
  "content": {
    "messages": [
      {"role": "user", "content": "How can I improve my sales pitch?"},
      {"role": "assistant", "content": "Here are some tips to improve your sales pitch:\n1. Understand your audience\n2. Highlight benefits, not features\n3. Use storytelling to engage\n4. Practice active listening\n5. Close with a clear call to action"}
    ],
    "context": {
      "previous_interactions": ["<previous_interaction_hash1>", "<previous_interaction_hash2>"]
    }
  },
  "sig": "<ETH signature>"
}
```

**Notes on `interaction_hash`**:
- The `interaction_hash` is a unique identifier for the current conversation event, typically generated by hashing the event's content (e.g., using SHA-256).
- This hash can be used to reference this specific interaction in future events, ensuring a verifiable sequence of interactions.

### 4.4 Session Management Event (Kind 30411)

Used to manage the lifecycle of conversation sessions, including creation, updates, and termination.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30411,
  "tags": [
    ["auth", "action=3", "key=30411", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "session"],
    ["session_id", "<session_id>"],
    ["action", "create"],  // or "update", "close"
    ["user_id", "<user_id>"],
    ["start_time", "<start_timestamp>"],
    ["end_time", "<end_timestamp>"]  // Only for "close" action
  ],
  "content": "{\"description\":\"Session for sales training\",\"metadata\":{\"topic\":\"Sales Techniques\"}}",
  "sig": "<ETH signature>"
}
```

---

## 5. Integration with Existing Subspaces

ModelGraph integrates with existing subspaces for a comprehensive model development and interaction workflow.

### 5.1 Project and Task Management

Utilizing the project (30101) and task (30102) events for organizing model development and AI interactions:

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
    ["sid", "0xMG"],
    ["project_id", "proj_001"],
    ["name", "Sales Assistant Model"],
    ["desc", "Fine-tuned model for sales conversation and training"],
    ["members", "0xAlice,0xBob"],
    ["status", "active"]
  ],
  "content": "",
  "sig": "<ETH signature>"
}
```

### 5.2 Model Governance

Utilizing governance events (30300-30304) for model-related decisions and AI interaction policies:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30301,
  "tags": [
    ["auth", "action=2", "key=30301", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "propose"],
    ["proposal_id", "model_upgrade_001"],
    ["rules", "min_votes>5,min_energy>1000"]
  ],
  "content": "Proposal to upgrade the model architecture for better performance and interaction quality",
  "sig": "<ETH signature>"
}
```

---

## 6. Complete Application Flow Examples

### 6.1 Model Development and Interaction Workflow

The workflow is expanded to include conversation and session management events, now with `interaction_hash` for conversation events:

```json
// 1. Create project
{
  "id": "<hash>",
  "pubkey": "<creator_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30101,
  "tags": [
    ["auth", "action=3", "key=30101", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "project"],
    ["sid", "0xMG"],
    ["project_id", "proj_001"],
    ["name", "Sales Assistant Model"],
    ["desc", "Fine-tuned model for sales conversation and training"],
    ["members", "0xAlice,0xBob"],
    ["status", "active"]
  ],
  "content": "",
  "sig": "<signature>"
}

// 2. Submit dataset
{
  "id": "<hash>",
  "pubkey": "<creator_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30405,
  "tags": [
    ["auth", "action=3", "key=30405", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "dataset"],
    ["sid", "0xMG"],
    ["project_id", "proj_001"],
    ["task_id", "task_001"],
    ["category", "Sales"],
    ["format", "jsonl"],
    ["contributors", "contributor_hash_fir,contributor_hash_sec"]
  ],
  "content": {
    "dataset": [
      {
        "messages": [
          {"role": "system", "content": "You are a customer service assistant"},
          {"role": "user", "content": "How to change delivery address?"},
          {"role": "assistant", "content": "You can follow these steps:\n1. Go to order details\n2. Click 'Change Address'\n3. Enter new address information\n4. Save changes"}
        ]
      },
      {
        "messages": [
          {"role": "system", "content": "You are a customer service assistant"},
          {"role": "user", "content": "How to specify delivery time?"},
          {"role": "assistant", "content": "You can follow these steps:\n1. Go to order details\n2. Click 'Change Delivery Time'\n3. Enter new delivery time\n4. Save changes"}
        ]
      }
    ],
    "generate": {
      "topic": ["AI Applications"],
      "num_samples": 3,
      "model_name": "Qwen/Qwen2.5-72B-Instruct-Turbo",
      "provider": "together_ai",
      "human_guidance": "Generate samples about AI applications in healthcare"
    }
  },
  "sig": "<signature>"
}

// 3. Create fine-tuning experiment
{
  "id": "<hash>",
  "pubkey": "<creator_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30409,
  "tags": [
    ["auth", "action=3", "key=30409", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "finetune"],
    ["sid", "0xFT"],
    ["project_id", "proj_001"],
    ["task_id", "task_001"],
    ["dataset_id", "<dataset_hash>"],
    ["provider_id", "<provider_hash>"]
  ],
  "content": "{\"name\":\"Fine-tuning Experiment 1\",\"description\":\"This is a test fine-tuning experiment\",\"hyperparameters\":{\"learning_rate\":0.001,\"epochs\":3}}",
  "sig": "<signature>"
}

// 4. Create a conversation session
{
  "id": "<hash>",
  "pubkey": "<creator_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30411,
  "tags": [
    ["auth", "action=3", "key=30411", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "session"],
    ["sid", "0xMG"],
    ["session_id", "session_001"],
    ["action", "create"],
    ["user_id", "<user_id>"],
    ["start_time", "<start_timestamp>"]
  ],
  "content": "{\"description\":\"Session for sales training\",\"metadata\":{\"topic\":\"Sales Techniques\"}}",
  "sig": "<signature>"
}

// 5. Record a conversation interaction with interaction_hash
{
  "id": "<hash>",
  "pubkey": "<creator_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30410,
  "tags": [
    ["auth", "action=3", "key=30410", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "conversation"],
    ["session_id", "session_001"],
    ["user_id", "<user_id>"],
    ["model_id", "<model_id>"],
    ["timestamp", "<interaction_timestamp>"],
    ["interaction_hash", "<current_interaction_hash>"]  // Unique hash for this interaction
  ],
  "content": {
    "messages": [
      {"role": "user", "content": "How can I improve my sales pitch?"},
      {"role": "assistant", "content": "Here are some tips to improve your sales pitch:\n1. Understand your audience\n2. Highlight benefits, not features\n3. Use storytelling to engage\n4. Practice active listening\n5. Close with a clear call to action"}
    ],
    "context": {
      "previous_interactions": ["<previous_interaction_hash1>", "<previous_interaction_hash2>"]
    }
  },
  "sig": "<signature>"
}

// 6. Close the conversation session
{
  "id": "<hash>",
  "pubkey": "<creator_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30411,
  "tags": [
    ["auth", "action=3", "key=30411", "exp=500000"],
    ["d", "subspace_op"],
    ["op", "session"],
    ["sid", "0xMG"],
    ["session_id", "session_001"],
    ["action", "close"],
    ["user_id", "<user_id>"],
    ["end_time", "<end_timestamp>"]
  ],
  "content": "{\"description\":\"Session for sales training closed\",\"metadata\":{\"topic\":\"Sales Techniques\"}}",
  "sig": "<signature>"
}

// 7. Propose model upgrade based on interaction data
{
  "id": "<hash>",
  "pubkey": "<creator_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30301,
  "tags": [
    ["auth", "action=2", "key=30301", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "propose"],
    ["proposal_id", "model_upgrade_001"],
    ["rules", "min_votes>5,min_energy>1000"]
  ],
  "content": "Proposal to upgrade the model architecture for better performance and interaction quality based on recent conversation data",
  "sig": "<signature>"
}
```