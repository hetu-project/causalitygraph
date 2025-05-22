## **Principles**

### Template-based

Creating a subspace is equivalent to creating a set of causality keys

### Composability

Any causality keys can be freely combined into a subspace

### Causality-Graph

A set of keys can be converted into VLC, representing causality

## Causality Key List

## General Keys

### Subspace Creation and Joining
| Kind Value | Event Name | Purpose | Key Tags Structure |
| --- | --- | --- | --- |
| 30100 | Subspace Create | Create a new subspace with defined operations and rules | ["d":"subspace_create", "sid", "subspace_name", "category", "ops", "rules"] |
| 30200 | Subspace Join | Allow a user to join an existing subspace | ["d":"subspace_join", "sid", "rules"] |

### Governance Subspace (CIP 01)

| Kind Value | Event Name | Purpose | Key Tags Structure |
| --- | --- | --- | --- |
| 30300 | Post | Publish content (e.g., announcements, documents) in the subspace | ["auth", "d":"subspace_op", "sid", "content_type", "parent"] |
| 30301 | Propose | Propose subspace rules or operations, requiring subsequent voting | ["auth", "d":"subspace_op", "sid", "proposal_id", "rules"] |
| 30302 | Vote | Vote on proposals for decentralized decision-making | ["auth", "d":"subspace_op", "sid", "proposal_id", "vote"] |
| 30303 | Invite | Invite new members to join the subspace | ["auth", "d":"subspace_op", "sid", "invitee_pubkey", "rules"] |
| 30304	| mint	| mint credit token, and issue to membership in community	| ["auth", "d":"subspace_op", "sid", "token_name", "token_symbol",”token_decimals”,”initial_supply”] |

### Common Graph Key (CIP 02)

| Kind Value | Event Name   | Purpose                                      | Key Tags Structure |
|------------|-------------|----------------------------------------------|--------------------|
| 30101      | Project     | Define a project within a subspace           | ["auth", "d":"subspace_op", "op":"project", "sid", "project_id", "name", "desc", "members", "status"] |
| 30102      | Task        | Task belonging to a project                   | ["auth", "d":"subspace_op", "op":"task", "sid", "project_id", "task_id", "title", "assignee", "status", "deadline", "priority"] |
| 30103      | Entity      | Define an entity node in a knowledge graph    | ["auth", "d":"subspace_op", "op":"entity", "sid", "entity_name", "entity_type"] |
| 30104      | Relation    | Define a relation between two entities        | ["auth", "d":"subspace_op", "op":"relation", "sid", "from", "to", "relation_type", "context", "weight", "description"] |
| 30105      | Observation | Attach an observation to an entity            | ["auth", "d":"subspace_op", "op":"observation", "sid", "entity_name", "observation"] |

### ModelGraph Subspace (CIP 03)

| Kind Value | Event Name      | Purpose                                         | Key Tags Structure                                                                 |
|------------|-----------------|-------------------------------------------------|------------------------------------------------------------------------------------|
| 30404 | Model | Submit a new model version | ["auth", "d":"subspace_op", "sid", "parent", "contrib"] |
| 30405 | Dataset | Submit or manage training datasets | ["auth", "d":"subspace_op", "op":"dataset", "sid", "project_id", "task_id", "category", "format", "contributors"] |
| 30406 | Compute | Submit computational tasks | ["auth", "d":"subspace_op", "sid", "compute_type"] |
| 30407 | Algo | Submit algorithm code or updates | ["auth", "d":"subspace_op", "sid", "algo_type"] |
| 30408 | Valid | Submit validation task results | ["auth", "d":"subspace_op", "sid", "valid_result"] |
| 30409 | Finetune | Create or update fine-tuning experiments | ["auth", "d":"subspace_op", "op":"finetune", "sid", "project_id", "task_id", "dataset_id", "provider_id"] |
| 30410 | Conversation | Record interactions between users and AI models | ["auth", "d":"subspace_op", "op":"conversation", "sid", "session_id", "user_id", "model_id", "timestamp", "interaction_hash"] |
| 30411 | Session | Manage the lifecycle of conversation sessions | ["auth", "d":"subspace_op", "op":"session", "sid", "session_id", "action", "user_id", "start_time", "end_time"] |

### Token Operations (CIP 04)

| Kind Value | Event Name | Purpose | Key Tags Structure |
| --- | --- | --- | --- |
| 30320 | Issue Token | Create and configure a new token | ["symbol", "name", "decimals"] |
| 30321 | Transfer | Transfer tokens between accounts | ["from", "to", "symbol", "amount"] |
| 30322 | Approve | Authorize another account to spend tokens | ["spender", "symbol", "amount"] |
| 30323 | MintCredit | Create new tokens based on predefined rules | ["symbol", "mint_if", "tag_key", "tag_value", "threshold", "mint_amount"] |

### OpenResearch Subspace (CIP 05)

| Kind Value | Event Name | Purpose | Key Tags Structure |
| --- | --- | --- | --- |
| 30501 | Paper | Submit or index a research paper | ["auth", "d":"subspace_op", "op":"paper", "sid", "doi", "paper_type", "authors"] |
| 30502 | Annotation | Create annotation on paper text | ["auth", "d":"subspace_op", "op":"annotation", "sid", "paper_id", "position", "type"] |
| 30503 | Review | Submit structured review of a paper | ["auth", "d":"subspace_op", "op":"review", "sid", "paper_id", "rating", "aspects"] |
| 30504 | AI_Analysis | Request or submit AI analysis results | ["auth", "d":"subspace_op", "op":"ai_analysis", "sid", "analysis_type", "paper_ids", "prompt"] |
| 30505 | Discussion | Create or contribute to research discussions | ["auth", "d":"subspace_op", "op":"discussion", "sid", "topic", "parent", "references"] |