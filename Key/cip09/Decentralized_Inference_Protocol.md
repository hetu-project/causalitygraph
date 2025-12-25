# CIP 09 — Prakasa Decentralized Inference Network Protocol (DINF)

This document defines the Nostr-based event and tag schema for **Prakasa**, a Decentralized Inference Network (DINF). It reuses the Flux-style `subspace_op` event model from CIP 08 and adds operation names and tags for:

- task publishing
- scheduler assignment and pipeline layout
- worker results and workload proofs
- settlement reporting and evaluation

All of these are designed to integrate cleanly with Parallax-style scheduling and layer allocation, while keeping the incentive layer (RIM) and on-chain settlement out of scope for this CIP.

## Base event structure

All events follow the Nostr / CausalityKey base used elsewhere:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash>",
  "pubkey": "<32 bytes lowercase hex-encoded pubkey of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": "<integer>",
  "tags": [
    ["auth", "action=<mask>", "key=<key-Id>", "exp=<expiration_clock>"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "<operation_name>"],
    ["<op_specific_key>", "<value>"]
  ],
  "content": "<arbitrary string, encrypted string, or JSON>",
  "sig": "<64 bytes lowercase hex-encoded signature>"
}
```

Common tag semantics (in addition to CIP 08):

- `auth`: Authorization mask and key reference (same as other CIPs).
- `d`: MUST be `"subspace_op"` to indicate a subspace operation.
- `sid`: Subspace / network id for indexing; e.g. `"prakasa-main"`.
- `op`: Operation name (see each kind below).
- `e`: References an event id (task / assignment / result linkage).
- `re`: References a related response/result/proof event id.
- `p`: Pubkey of a participant (scheduler, worker, or user); can appear multiple times.
- `account`: Optional settlement account (e.g., EVM address) for the participant.

Prakasa-specific design principles:

- All control and observability data flows over Nostr events.
- User task requests MUST have encrypted content (so only authorized schedulers or recipients can read task inputs).
- Scheduler nodes are a known / pre-configured set of participants responsible for model partitioning and task assignment.
- Worker nodes publish signed workload proofs and results to Nostr for later settlement (RIM or other incentive systems).
- All fields required to reconstruct **who did what work on which layers, for which task** are included in the DINF events.

## Event kinds and operation names

We allocate the 30900–30910 range for Prakasa DINF events. Implementers may change numeric kinds but MUST match `op` strings when parsing.

The main flow:

1. Task publish: user sends `dinf_task_publish`.
2. Scheduler assignment: scheduler sends `dinf_task_assign` describing pipelines and per-worker layer ranges.
3. Worker result: each worker sends `dinf_task_result` with outputs and basic metrics.
4. Workload proof: each worker sends `dinf_workload_proof` with normalized work units and reproducibility metadata.
5. Settlement: scheduler or settlement agent sends `dinf_settlement` with per-worker allocations.
6. Evaluation / dispute (optional): validators or schedulers send `dinf_evaluation`.

### 30900 — Decentralized Task Publish (op: `dinf_task_publish`)

Purpose: Publish an encrypted inference task intent into the Prakasa network.

Typical tags:

- `auth`, `d:subspace_op`, `sid`, `op:dinf_task_publish`
- `power_cost`: estimated cost to execute (semantic units)
- `reward`: short descriptor or JSON reward description (can reference RIM / on-chain logic)
- `deadline`: Unix timestamp for task expiry
- `category`: `inference|analysis|generation|other`
- `difficulty`: `easy|medium|hard`
- `p`: optional invited scheduler pubkeys (omit to allow any scheduler)
- `rules`: optional pointer to task-specific or subspace-wide evaluation / settlement rules

Content: MUST be an **encrypted JSON string**. The plaintext JSON SHOULD include:

- `description`: short human description of the task
- `input_cipher`: base64 or ciphertext envelope intended for scheduler(s) public key
- `model_spec`: model family / version / layer hints (e.g., `{"model_name":"Llama-3-70B","num_layers":80}`)
- `scoring`: evaluation criteria (optional)
- `settlement_prefs`: optional hints for RIM or other settlement logic, e.g. reward curve, max budget

Example (conceptual):

```json
{
  "kind": 30900,
  "pubkey": "<user_pubkey>",
  "created_at": 1710000006,
  "tags": [
    ["auth", "action=2", "key=30900", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "prakasa-main"],
    ["op", "dinf_task_publish"],
    ["power_cost", "120"],
    ["reward", "{\"type\":\"flux\",\"amount\":\"500\"}"],
    ["deadline", "1710400000"],
    ["category", "inference"],
    ["difficulty", "medium"]
  ],
  "content": "<encrypted JSON blob>",
  "sig": "<signature>"
}
```

### 30901 — Scheduler Assignment (op: `dinf_task_assign`)

Purpose: Scheduler announces the intra-model partitioning, pipeline layout, and worker assignment for a published task. This maps closely to Parallax `Scheduler`, `LayerAllocator`, and `Node` structures.

Typical tags:

- `auth`, `d:subspace_op`, `sid`, `op:dinf_task_assign`
- `e`: task event id (the original `dinf_task_publish` id)
- multiple `p` tags: assigned worker pubkeys
- `model_name`: model identifier (e.g., `Llama-3-70B`)
- `model_version`: optional model version / hash
- `pipeline_id`: optional id for this end-to-end pipeline
- `allocation_id`: deterministic id for this assignment (used later by proofs and settlement)

The per-worker layer and parallelism details are carried in `content` as JSON instead of overloading tags. A minimal schema:

```json
{
  "assignments": [
    {
      "worker_pubkey": "<worker1_pubkey>",
      "node_id": "<scheduler_internal_node_id>",
      "account": "<optional_worker_evm_or_other_account>",
      "start_layer": 0,
      "end_layer": 10,
      "tp_rank": 0,
      "tp_size": 2,
      "dp_rank": 0,
      "dp_size": 1,
      "max_concurrent_requests": 8,
      "max_sequence_length": 4096,
      "expected_work_units": 1000
    }
  ],
  "model": {
    "model_name": "Llama-3-70B",
    "num_layers": 80
  },
  "routing": {
    "pipeline_id": "pipe-1",
    "node_path": ["node-a", "node-b", "node-c"]
  },
  "deadline": 1710400000
}
```

Conceptual example event:

```json
{
  "kind": 30901,
  "pubkey": "<scheduler_pubkey>",
  "created_at": 1710000007,
  "tags": [
    ["auth", "action=2", "key=30901", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "prakasa-main"],
    ["op", "dinf_task_assign"],
    ["e", "<task_event_id>"],
    ["pipeline_id", "pipe-1"],
    ["allocation_id", "alloc-abc123"],
    ["model_name", "Llama-3-70B"],
    ["model_version", "v1"]
  ],
  "content": "{\"assignments\":[{\"worker_pubkey\":\"<worker1_pubkey>\",\"node_id\":\"node-a\",\"account\":\"0xworker1\",\"start_layer\":0,\"end_layer\":10,\"tp_rank\":0,\"tp_size\":2}],\"model\":{\"model_name\":\"Llama-3-70B\",\"num_layers\":80},\"routing\":{\"pipeline_id\":\"pipe-1\",\"node_path\":[\"node-a\",\"node-b\",\"node-c\"]},\"deadline\":1710400000}",
  "sig": "<signature>"
}
```

### 30902 — Worker Result / Response (op: `dinf_task_result`)

Purpose: Worker publishes computed outputs and execution metadata. This is the “response” for a single shard of the model (e.g., a specific layer range) and maps to Parallax executor output.

Typical tags:

- `auth`, `d:subspace_op`, `sid`, `op:dinf_task_result`
- `e`: task event id
- `re`: assignment event id (links to `dinf_task_assign`)
- `p`: scheduler pubkey
- `account`: worker settlement account (e.g., from Parallax `Node.account`)
- `status`: `partial|completed|failed`

Execution-related tags (for quick indexing; full metrics live in `content`):

- `start_layer`, `end_layer`: half-open layer range processed on this worker
- `tokens_in`: prompt tokens processed
- `tokens_out`: tokens generated
- `latency_ms`: total wall-clock latency in milliseconds for this shard

Content: JSON or pointer to encrypted blob containing:

- `output_hash`: hash of model outputs (or pointer to ciphertext)
- `output_pointer`: optional URL / IPFS / blobstore reference
- `metrics`: detailed metrics such as:
  - `tokens_in`, `tokens_out`, `batch_size`
  - `wall_time_ms`, `compute_time_ms`, `io_time_ms`
  - `estimated_flops`, `kv_cache_bytes`
- `evidence`: optional reference to `dinf_workload_proof` events or internal logs

Example:

```json
{
  "kind": 30902,
  "pubkey": "<worker_pubkey>",
  "created_at": 1710000008,
  "tags": [
    ["auth", "action=2", "key=30902", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "prakasa-main"],
    ["op", "dinf_task_result"],
    ["e", "<task_event_id>"],
    ["re", "<assignment_event_id>"],
    ["p", "<scheduler_pubkey>"],
    ["account", "0xworker1"],
    ["start_layer", "0"],
    ["end_layer", "10"],
    ["tokens_in", "2048"],
    ["tokens_out", "128"],
    ["latency_ms", "950"],
    ["status", "completed"]
  ],
  "content": "{\"output_hash\":\"0xabc...\",\"metrics\":{\"tokens_in\":2048,\"tokens_out\":128,\"batch_size\":4,\"wall_time_ms\":950,\"estimated_flops\":1.2e13,\"kv_cache_bytes\":8589934592}}",
  "sig": "<signature>"
}
```

### 30903 — Workload Proof (op: `dinf_workload_proof`)

Purpose: Worker publishes a signed, content-addressed proof of computation so that later settlement (RIM or other systems) can compute payouts. This is the canonical place to put normalized “work units” derived from Parallax scheduler / model info.

Typical tags:

- `auth`, `d:subspace_op`, `sid`, `op:dinf_workload_proof`
- `e`: task event id
- `re`: result event id (links to `dinf_task_result`)
- `p`: worker pubkey
- `account`: worker settlement account (e.g. EVM address from `Node.account`)
- `allocation_id`: assignment id from `dinf_task_assign`
- `work_units`: normalized numeric units completed (e.g., FLOPs / 1e9 or `layer_tokens`)
- `score`: optional verifier score or quality multiplier

Content: JSON with proof artifacts and metrics, for example:

```json
{
  "worker_pubkey": "<worker_pubkey>",
  "node_id": "<scheduler_internal_node_id>",
  "account": "0xworker1",
  "model": {
    "model_name": "Llama-3-70B",
    "num_layers": 80
  },
  "layer_range": {
    "start_layer": 0,
    "end_layer": 10
  },
  "metrics": {
    "tokens_in": 2048,
    "tokens_out": 128,
    "batch_size": 4,
    "wall_time_ms": 950,
    "estimated_flops": 1.2e13,
    "kv_cache_bytes": 8589934592,
    "work_units": 1200
  },
  "proof": {
    "bundle_hash": "bafy...ipfs",
    "merkle_root": "0xdeadbeef...",
    "logs_pointer": "ipfs://bafy...",
    "replay_hint": "command line / docker image / model hash"
  }
}
```

Example event:

```json
{
  "kind": 30903,
  "pubkey": "<worker_pubkey>",
  "created_at": 1710000009,
  "tags": [
    ["auth", "action=2", "key=30903", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "prakasa-main"],
    ["op", "dinf_workload_proof"],
    ["e", "<task_event_id>"],
    ["re", "<result_event_id>"],
    ["p", "<worker_pubkey>"],
    ["account", "0xworker1"],
    ["allocation_id", "alloc-abc123"],
    ["work_units", "1200"],
    ["score", "1.0"]
  ],
  "content": "{\"worker_pubkey\":\"<worker_pubkey>\",\"node_id\":\"node-a\",\"account\":\"0xworker1\",\"model\":{\"model_name\":\"Llama-3-70B\",\"num_layers\":80},\"layer_range\":{\"start_layer\":0,\"end_layer\":10},\"metrics\":{\"tokens_in\":2048,\"tokens_out\":128,\"batch_size\":4,\"wall_time_ms\":950,\"estimated_flops\":1.2e13,\"kv_cache_bytes\":8589934592,\"work_units\":1200},\"proof\":{\"bundle_hash\":\"bafy...ipfs\",\"merkle_root\":\"0xdeadbeef...\",\"logs_pointer\":\"ipfs://bafy...\",\"replay_hint\":\"docker://image@sha256:...\"}}",
  "sig": "<signature>"
}
```

### 30904 — Settlement Report (op: `dinf_settlement`)

Purpose: Scheduler or settlement agent aggregates workload proofs and publishes settlement instructions or results. This event is the bridge from Prakasa’s Nostr workload graph to any on-chain or off-chain incentive mechanism.

Typical tags:

- `auth`, `d:subspace_op`, `sid`, `op:dinf_settlement`
- `e`: task event id
- `re`: optional reference to a previous settlement / evaluation event
- multiple `p` tags: workers being paid
- `reward_total`: total reward for the task (numeric)
- `currency`: settlement currency symbol or identifier (e.g., `FLUX`, `ETH`)
- `settlement_ref`: optional external settlement tx hash / pointer

Content: Signed JSON ledger of per-worker allocations, for example:

```json
{
  "task_id": "<task_event_id>",
  "allocation_id": "alloc-abc123",
  "reward_total": "500",
  "currency": "FLUX",
  "workers": [
    {
      "worker_pubkey": "<worker1_pubkey>",
      "account": "0xworker1",
      "proof_event_id": "<workload_proof_event_id_1>",
      "work_units": 1200,
      "effective_score": 1.0,
      "share": 0.6,
      "amount": "300"
    },
    {
      "worker_pubkey": "<worker2_pubkey>",
      "account": "0xworker2",
      "proof_event_id": "<workload_proof_event_id_2>",
      "work_units": 800,
      "effective_score": 0.9,
      "share": 0.4,
      "amount": "200"
    }
  ],
  "proof_refs": ["<workload_proof_event_id_1>", "<workload_proof_event_id_2>"],
  "settlement": {
    "tx_hash": "0xsettlementtx...",
    "chain_id": 1,
    "contract": "0xrimcontract..."
  }
}
```

Example event:

```json
{
  "kind": 30904,
  "pubkey": "<scheduler_or_settlement_agent_pubkey>",
  "created_at": 1710000010,
  "tags": [
    ["auth", "action=2", "key=30904", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "prakasa-main"],
    ["op", "dinf_settlement"],
    ["e", "<task_event_id>"],
    ["reward_total", "500"],
    ["currency", "FLUX"],
    ["p", "<worker1_pubkey>"],
    ["p", "<worker2_pubkey>"]
  ],
  "content": "{\"task_id\":\"<task_event_id>\",\"allocation_id\":\"alloc-abc123\",\"reward_total\":\"500\",\"currency\":\"FLUX\",\"workers\":[{\"worker_pubkey\":\"<worker1_pubkey>\",\"account\":\"0xworker1\",\"proof_event_id\":\"<workload_proof_event_id_1>\",\"work_units\":1200,\"effective_score\":1.0,\"share\":0.6,\"amount\":\"300\"},{\"worker_pubkey\":\"<worker2_pubkey>\",\"account\":\"0xworker2\",\"proof_event_id\":\"<workload_proof_event_id_2>\",\"work_units\":800,\"effective_score\":0.9,\"share\":0.4,\"amount\":\"200\"}],\"proof_refs\":[\"<workload_proof_event_id_1>\",\"<workload_proof_event_id_2>\"],\"settlement\":{\"tx_hash\":\"0xsettlementtx...\",\"chain_id\":1,\"contract\":\"0xrimcontract...\"}}",
  "sig": "<signature>"
}
```

### 30905 — Evaluation / Dispute (op: `dinf_evaluation`)

Purpose: Validators, users, or schedulers publish human or automated evaluations of results; used to accept/reject work for settlement or to trigger disputes.

Typical tags:

- `auth`, `d:subspace_op`, `sid`, `op:dinf_evaluation`
- `e`: task event id
- `re`: response or proof event id (e.g., `dinf_task_result` or `dinf_workload_proof`)
- `p`: worker pubkey being evaluated
- `score`: numeric score (0–100 or 0–1)
- `status`: `approved|rejected|needs_revision|disputed`

Content: JSON feedback payload:

```json
{
  "feedback": "Detailed evaluation feedback",
  "strengths": "Strengths or positive aspects",
  "improvements": "Improvement suggestions",
  "evidence": "Pointers to evidence or test cases",
  "verifier_type": "human|auto|committee",
  "reason_code": "policy_violation|low_quality|timeout|other"
}
```

## Integration notes for Parallax / Prakasa

This protocol is intended to map directly onto Parallax components and their scheduling model:

- Scheduler orchestration and allocation: `src/parallax/server/scheduler.py`
- Layer allocation and pipelines: `src/scheduling/layer_allocation.py`
- Node hardware and per-layer capacity / latency: `src/scheduling/node.py` and `src/scheduling/model_info.py`
- Request routing and pipeline discovery: `src/scheduling/request_routing.py`
- Executor / worker process lifecycle and runner: `src/parallax/server/executor/factory.py`
- P2P server / node bootstrap: `src/parallax/p2p/server.py`
- Shared state and layer allocation: `src/parallax/utils/shared_state.py`
- Task launching and orchestration: `src/parallax/launch.py`

Recommended mapping:

- Scheduler processes subscribe to Nostr relay(s) and index `dinf_task_publish` events for their `sid`.
- On assignment, schedulers publish `dinf_task_assign` describing per-worker `start_layer` / `end_layer` ranges, pipeline ids, and expected work units derived from the Parallax `LayerAllocator` and `Node` state.
- Worker processes (Parallax executors) monitor Nostr for `dinf_task_assign` events addressed to them (via `p` tags and/or internal `node_id`) and then perform computation.
- After computation, workers MUST publish `dinf_task_result` and `dinf_workload_proof` with work units and layer ranges that can be reconciled with the scheduler’s assignment.
- A settlement agent or scheduler then aggregates proofs and emits `dinf_settlement`, which can be consumed by RIM or any other incentive mechanism.

## Security, privacy and encryption

- Task input payloads MUST be encrypted. Recommended pattern: encrypt content to the scheduler group public key(s) (or ephemeral keys) and place ciphertext in `content`.
- Sensitive outputs can be encrypted to the scheduler / user or published as ciphertext pointers.
- All events must be signed (Nostr standard). Use `auth` tags to indicate authority or required action masks.
- Relayers SHOULD enforce `auth` policies for events that change balances or allocate rewards (e.g., `dinf_settlement`).
- Implementers SHOULD validate that `dinf_workload_proof` events are internally consistent with:
  - `dinf_task_assign` layer ranges,
  - model metadata (e.g., `ModelInfo` for FLOPs / KV estimates),
  - and any configured settlement rules for the subspace.

## Indexing and queries

For Prakasa indexers and explorers:

- Index by `sid` and `op` for efficient filtering per subspace and operation type.
- Index `e` and `re` tags to gather the full chain:
  - `task_publish` → `task_assign` → `task_result` → `workload_proof` → `settlement` → `evaluation`.
- Index `account` and `p` for per-worker history and accounting.
- Index `allocation_id`, `pipeline_id`, and `layer_range` (from content) for reconstruction of execution pipelines.

## Extensions and advanced flows

- Multi-scheduler coordination: schedulers may coordinate via Nostr `subspace_op` events to hand-off or re-balance tasks across multiple Prakasa clusters.
- RIM / incentive flows: settlement agents or smart-contract bridges subscribe to `dinf_settlement` and trigger on-chain transfers (RIM or other protocols).
- Reputation and slashing: additional ops can be defined to maintain on-chain or off-chain reputation based on `dinf_evaluation` and `dinf_workload_proof`.

## Compatibility

- Keep `op` strings stable to enable tooling compatibility:
  - `dinf_task_publish`
  - `dinf_task_assign`
  - `dinf_task_result`
  - `dinf_workload_proof`
  - `dinf_settlement`
  - `dinf_evaluation`
- Numeric kinds MAY vary between deployments, but SHOULD remain within a reserved range (30900–30910) for the base specification.