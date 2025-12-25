# CIP 08 — Intent publish and Task Mining Protocol (Draft)

This document defines the Intent publish and Task Mining protocol family for the causality key (CIP 08). It adapts the Flux-style `subspace_op` event model and common tags used in other CIPs so that task creation, submission, and evaluation work consistently across subspaces.

## Overview

The Task Mining protocol supports three task publishing types and two follow-up flows:

- Direct Intent Task (Kind 30800)
- Causal Graph Task (Kind 30801)
- Simple Event / Single-Event Task (Kind 30802)
- Task Response (Kind 30803)
- Task Evaluation (Kind 30804)

All events in this CIP follow the Nostr / CausalityKey event base used elsewhere:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": "<integer>",
  "tags": [
    ["auth", "action=<mask>", "key=<key-Id>", "exp=<expiration_clock>"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "<operation_name>"],
    ["parent", "<parent_event_id>"],
    ["rules", "<optional_rules_reference>"],
    ["<op_specific_key>", "<value>"]
  ],
  "content": "<arbitrary string or JSON>",
  "sig": "<64 bytes lowercase hex-encoded ETH signature>"
}
```

Common tag semantics

- `auth`: Authorization mask and key reference (same as other CIPs).
- `d`: Must be `"subspace_op"` to indicate a subspace operation.
- `sid`: Subspace ID where this task is published.
- `op`: Short operation name describing the intent (see each kind below).
- `parent`: Optional parent event id (used when task is derived from another object).
- `rules`: Optional pointer to subspace rules or evaluation policies.

## Event Kinds and Operation Names

All kinds use the `subspace_op` structure and include `sid` and `op` tags. The `op` values are normative strings implementers should match when parsing.

## 30800 — Direct Intent Task (op: `flux_task_publish_direct`)

Purpose: Publish a direct task intent with explicit requirements and reward. Suitable for computational, creative or human-in-the-loop tasks where the creator specifies inputs, outputs, scoring and deadlines.

Typical tags:

- `auth`, `d:subspace_op`, `sid`, `op:flux_task_publish_direct`
- `power_cost`: Power cost required to submit or accept the task
- `reward`: Short descriptor or reference to reward details (could be a link or token symbol)
- `deadline`: Unix timestamp for task expiry
- `category`: `computation|analysis|creative|other`
- `difficulty`: `easy|medium|hard`
- `p`: optional list of invited participant pubkeys

Example event (kind 30800):

```json
{
  "kind": 30800,
  "pubkey": "<task_creator_pubkey>",
  "created_at": 1710000006,
  "tags": [
    ["auth", "action=2", "key=30800", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux_task_publish_direct"],
    ["power_cost", "100"],
    ["reward", "{\"type\":\"flux\",\"amount\":\"500\"}"],
    ["deadline", "1710400000"],
    ["category", "computation"],
    ["difficulty", "medium"]
  ],
  "content": "{\"description\": \"Task description\", \"requirements\": \"Completion requirements\", \"evaluation_criteria\": \"Evaluation criteria\"}",
  "sig": "<signature>"
}
```

## 30801 — Causal Graph Task (op: `flux_task_publish_causal`)

Purpose: Describe a task whose inputs or outputs are a causal graph (set of events and relations). Useful when scoring or task semantics depend on the causal structure of multiple events.

Typical tags:

- `auth`, `d:subspace_op`, `sid`, `op:flux_task_publish_causal`
- `power_cost`, `graph` (reference/hash to causal graph payload), `e` (root/related events), `p` (participant pubkeys)

Example event (kind 30801):

```json
{
  "kind": 30801,
  "pubkey": "<task_creator_pubkey>",
  "created_at": 1710000007,
  "tags": [
    ["auth", "action=2", "key=30801", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux_task_publish_causal"],
    ["power_cost", "150"],
    ["graph", "<causal_graph_hash>"],
    ["e", "<root_event_id1>"],
    ["e", "<root_event_id2>"],
    ["p", "<participant1>"],
    ["p", "<participant2>"]
  ],
  "content": "{\"causal_graph\": {\"nodes\": [{\"id\": \"node1\", \"type\": \"message\", \"creator\": \"pubkey\"}], \"edges\": [{\"from\": \"node1\", \"to\": \"node2\", \"type\": \"reply\"}]}, \"scoring_rules\": \"Contribution scoring rules\"}",
  "sig": "<signature>"
}
```

## 30802 — Single Event Task (op: `flux_task_publish_single`)

Purpose: Publish a task that targets a single existing event (e.g., ask to analyze, summarize or extend an event). This is a lightweight pattern for targeted micro-tasks.

Typical tags:

- `auth`, `d:subspace_op`, `sid`, `op:flux_task_publish_single`
- `power_cost`, `reward`, `deadline`, `category`, `difficulty`, `e` (target event), `p` (target author), `input_type`, `output_type`

Example event (kind 30802):

```json
{
  "kind": 30802,
  "pubkey": "<task_creator_pubkey>",
  "created_at": 1710000008,
  "tags": [
    ["auth", "action=2", "key=30802", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux_task_publish_single"],
    ["power_cost", "80"],
    ["reward", "{\"type\":\"flux\",\"amount\":\"200\"}"],
    ["deadline", "1710400000"],
    ["category", "analysis"],
    ["difficulty", "easy"],
    ["e", "<target_event_id>"],
    ["p", "<target_event_author>"],
    ["input_type", "text"],
    ["output_type", "analysis"]
  ],
  "content": "{\"description\": \"Task description\", \"requirements\": \"Processing requirements and standards\", \"evaluation\": {\"quality_criteria\": \"Quality criteria\", \"completion_metrics\": \"Completion metrics\"}, \"examples\": \"Example of expected output\"}",
  "sig": "<signature>"
}
```

## 30803 — Task Response (op: `flux_task_response`)

Purpose: A participant submits a response or solution for a published task. Responses should reference the originating task event via `e` and include any necessary evidence in `content`.

Typical tags:

- `auth`, `d:subspace_op`, `sid`, `op:flux_task_response`
- `e` (task id), `p` (task creator), `status` (`submitted|completed|failed`), `power_cost` (response cost), `quality_score` (optional self-assessed)

Example event (kind 30803):

```json
{
  "kind": 30803,
  "pubkey": "<worker_pubkey>",
  "created_at": 1710000009,
  "tags": [
    ["auth", "action=2", "key=30803", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux_task_response"],
    ["e", "<task_event_id>"],
    ["p", "<task_creator_pubkey>"],
    ["status", "submitted"],
    ["power_cost", "40"],
    ["quality_score", "78"]
  ],
  "content": "{\"solution\": \"Task solution\", \"evidence\": \"Completion proof\", \"metadata\": {\"processing_time\": \"120s\", \"resources_used\": \"cpu:2,mem:512MB\"}}",
  "sig": "<signature>"
}
```

## 30804 — Task Evaluation (op: `flux_task_evaluation`)

Purpose: Validators evaluate submitted responses. Evaluations reference both the task event and the response event, and produce a score, status, and optional feedback.

Typical tags:

- `auth`, `d:subspace_op`, `sid`, `op:flux_task_evaluation`
- `e` (task id), `re` (response id), `p` (worker pubkey), `score`, `status` (`approved|rejected|needs_revision`), `power_cost` (evaluation cost)

Example event (kind 30804):

```json
{
  "kind": 30804,
  "pubkey": "<validator_pubkey>",
  "created_at": 1710000010,
  "tags": [
    ["auth", "action=2", "key=30804", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux_task_evaluation"],
    ["e", "<task_event_id>"],
    ["re", "<response_event_id>"],
    ["p", "<worker_pubkey>"],
    ["score", "85"],
    ["status", "approved"],
    ["power_cost", "5"]
  ],
  "content": "{\"feedback\": \"Detailed evaluation feedback\", \"strengths\": \"Strengths\", \"improvements\": \"Improvement suggestions\", \"evidence\": \"Evaluation basis\"}",
  "sig": "<signature>"
}
```

## Querying and Implementer Notes

- Implementers should index tasks by `sid` and `op` to efficiently filter task lists per subspace and operation type.
- `power_cost` is treated as a semantic token: it indicates how much Flux Power (or other resource) is required or consumed when participating.
- `reward` may be a structured JSON string in the `tags` or a pointer to a richer reward definition in `content` or an external resource.
- For `flux_task_publish_causal`, the `graph` tag references a canonical payload (IPFS hash, content-addressed blob) that describes nodes and edges; validators should fetch and validate this graph as needed.

## Extensions and Security

- `auth` tags must be enforced by subspace relayers: only keys with proper write/mint rights may publish tasks that incur costs or allocate rewards.
- Moderation and dispute resolution can be built as separate `op` flows (not part of this initial draft).

## Compatibility

This CIP follows the same `subspace_op` event model used by Flux and other CIPs — keep the `op` strings stable across implementations to enable generic tooling.

