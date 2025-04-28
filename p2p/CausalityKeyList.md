## **Principles**

### Template-based

Creating a subspace is equivalent to creating a set of causality keys

### Composability

Any causality keys can be freely combined into a subspace

### Causality-Graph

A set of keys can be converted into VLC, representing causality

## Causality Key List

## General Keys

| Kind Value | Event Name | Purpose | Key Tags Structure |
| --- | --- | --- | --- |
| 30100 | Subspace Create | Create a new subspace with defined operations and rules | ["d":"subspace_create", "sid", "subspace_name", "ops", "rules"] |
| 30200 | Subspace Join | Allow a user to join an existing subspace | ["d":"subspace_join", "sid", "rules"] |

### Governance Subspace (CIP 01)

| Kind Value | Event Name | Purpose | Key Tags Structure |
| --- | --- | --- | --- |
| 30300 | Post | Publish content (e.g., announcements, documents) in the subspace | ["auth", "d":"subspace_op", "sid", "content_type", "parent"] |
| 30301 | Propose | Propose subspace rules or operations, requiring subsequent voting | ["auth", "d":"subspace_op", "sid", "proposal_id", "rules"] |
| 30302 | Vote | Vote on proposals for decentralized decision-making | ["auth", "d":"subspace_op", "sid", "proposal_id", "vote"] |
| 30303 | Invite | Invite new members to join the subspace | ["auth", "d":"subspace_op", "sid", "invitee_pubkey", "rules"] |

### ModelGraph Subspace (CIP 02)

| 30304 | Model | Submit a new model version | ["auth", "d":"subspace_op", "sid", "parent", "contrib"] |
| --- | --- | --- | --- |
| 30305 | Data | Submit training datasets | ["auth", "d":"subspace_op", "sid", "size"] |
| 30306 | Compute | Submit computational tasks | ["auth", "d":"subspace_op", "sid", "compute_type"] |
| 30307 | Algo | Submit algorithm code or updates | ["auth", "d":"subspace_op", "sid", "algo_type"] |
| 30308 | Valid | Submit validation task results | ["auth", "d":"subspace_op", "sid", "valid_result"] |