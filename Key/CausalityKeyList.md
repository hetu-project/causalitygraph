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

| Kind Value | Event Name      | Purpose                                                 | Key Tags Structure                                                          |
| ---------- | --------------- | ------------------------------------------------------- | --------------------------------------------------------------------------- |
| 30100      | Subspace Create | Create a new subspace with defined operations and rules | ["d":"subspace_create", "sid", "subspace_name", "category", "ops", "rules"] |
| 30200      | Subspace Join   | Allow a user to join an existing subspace               | ["d":"subspace_join", "sid", "rules"]                                       |

### Governance Subspace (CIP 01)

| Kind Value | Event Name | Purpose                                                           | Key Tags Structure                                                                                 |
| ---------- | ---------- | ----------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| 30300      | Post       | Publish content (e.g., announcements, documents) in the subspace  | ["auth", "d":"subspace_op", "sid", "content_type", "parent"]                                       |
| 30301      | Propose    | Propose subspace rules or operations, requiring subsequent voting | ["auth", "d":"subspace_op", "sid", "proposal_id", "rules"]                                         |
| 30302      | Vote       | Vote on proposals for decentralized decision-making               | ["auth", "d":"subspace_op", "sid", "proposal_id", "vote"]                                          |
| 30303      | Invite     | Invite new members to join the subspace                           | ["auth", "d":"subspace_op", "sid", "invitee_pubkey", "rules"]                                      |
| 30304      | mint       | mint credit token, and issue to membership in community           | ["auth", "d":"subspace_op", "sid", "token_name", "token_symbol",”token_decimals”,”initial_supply”] |

### Common Graph Key (CIP 02)

| Kind Value | Event Name  | Purpose                                    | Key Tags Structure                                                                                                              |
| ---------- | ----------- | ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| 30101      | Project     | Define a project within a subspace         | ["auth", "d":"subspace_op", "op":"project", "sid", "project_id", "name", "desc", "members", "status"]                           |
| 30102      | Task        | Task belonging to a project                | ["auth", "d":"subspace_op", "op":"task", "sid", "project_id", "task_id", "title", "assignee", "status", "deadline", "priority"] |
| 30103      | Entity      | Define an entity node in a knowledge graph | ["auth", "d":"subspace_op", "op":"entity", "sid", "entity_name", "entity_type"]                                                 |
| 30104      | Relation    | Define a relation between two entities     | ["auth", "d":"subspace_op", "op":"relation", "sid", "from", "to", "relation_type", "context", "weight", "description"]          |
| 30105      | Observation | Attach an observation to an entity         | ["auth", "d":"subspace_op", "op":"observation", "sid", "entity_name", "observation"]                                            |

### ModelGraph Subspace (CIP 03)

| Kind Value | Event Name   | Purpose                                         | Key Tags Structure                                                                                                            |
| ---------- | ------------ | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| 30404      | Model        | Submit a new model version                      | ["auth", "d":"subspace_op", "sid", "parent", "contrib"]                                                                       |
| 30405      | Dataset      | Submit or manage training datasets              | ["auth", "d":"subspace_op", "op":"dataset", "sid", "project_id", "task_id", "category", "format", "contributors"]             |
| 30406      | Compute      | Submit computational tasks                      | ["auth", "d":"subspace_op", "sid", "compute_type"]                                                                            |
| 30407      | Algo         | Submit algorithm code or updates                | ["auth", "d":"subspace_op", "sid", "algo_type"]                                                                               |
| 30408      | Valid        | Submit validation task results                  | ["auth", "d":"subspace_op", "sid", "valid_result"]                                                                            |
| 30409      | Finetune     | Create or update fine-tuning experiments        | ["auth", "d":"subspace_op", "op":"finetune", "sid", "project_id", "task_id", "dataset_id", "provider_id"]                     |
| 30410      | Conversation | Record interactions between users and AI models | ["auth", "d":"subspace_op", "op":"conversation", "sid", "session_id", "user_id", "model_id", "timestamp", "interaction_hash"] |
| 30411      | Session      | Manage the lifecycle of conversation sessions   | ["auth", "d":"subspace_op", "op":"session", "sid", "session_id", "action", "user_id", "start_time", "end_time"]               |

### Flux Credit Operations (CIP 04)

| Kind Value | Event Name            | Purpose                               | Key Tags Structure                                                                                                                            |
| ---------- | --------------------- | ------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 30320      | Flux Power Issue      | Create and configure a new Flux Power | ["auth", "d":"subspace_op", "op":"flux20_issue", "sid", "symbol", "name", "total_supply", "decimals", "transferable", "mintable"]             |
| 30321      | Flux Power Allocation | Allocate Power to recipients          | ["auth", "d":"subspace_op", "op":"flux20_allocation", "sid", "symbol", "p", "amount", "purpose", "vesting"]                                   |
| 30322      | Flux Power Burn       | Consume/burn Power                    | ["auth", "d":"subspace_op", "op":"flux20_burn", "sid", "symbol", "amount", "action", "ref", "balance"]                                        |
| 30323      | Flux Power Transfer   | Transfer Power between accounts       | ["auth", "d":"subspace_op", "op":"flux20_transfer", "sid", "symbol", "from", "to", "amount"]                                                  |
| 30330      | Flux Credit SBT       | Issue Credit SBT for reputation       | ["auth", "d":"subspace_op", "op":"flux21_credit", "sid", "p", "symbol", "score", "level", "category", "transferable"]                         |
| 30331      | Flux Bond SBT         | Issue Bond SBT for staking            | ["auth", "d":"subspace_op", "op":"flux21_bond", "sid", "p", "symbol", "amount", "purpose", "start_time", "expiry", "penalty", "transferable"] |
| 30332      | Flux SBT Update       | Update existing Credit or Bond SBT    | ["auth", "d":"subspace_op", "op":"flux21_update", "sid", "e", "p", "symbol", "new_score", "new_amount", "new_level", "reason", "evidence"]    |

### OpenResearch Subspace (CIP 05)

| Kind Value | Event Name    | Purpose                                       | Key Tags Structure                                                                                |
| ---------- | ------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| 30501      | Paper         | Submit or index a research paper              | ["auth", "d":"subspace_op", "op":"paper", "sid", "doi", "paper_type", "authors"]                  |
| 30502      | Annotation    | Create annotation on paper text               | ["auth", "d":"subspace_op", "op":"annotation", "sid", "paper_id", "position", "type"]             |
| 30503      | Review        | Submit structured review of a paper           | ["auth", "d":"subspace_op", "op":"review", "sid", "paper_id", "rating", "aspects"]                |
| 30504      | AI_Analysis   | Request or submit AI analysis results         | ["auth", "d":"subspace_op", "op":"ai_analysis", "sid", "analysis_type", "paper_ids", "prompt"]    |
| 30505      | Discussion    | Create or contribute to research discussions  | ["auth", "d":"subspace_op", "op":"discussion", "sid", "topic", "parent", "references"]            |
| 30506      | ReadPaper     | Record reading behavior and depth for a paper | ["auth", "d":"subspace_op", "op":"read_paper", "sid", "paper_id", "user_id", "duration", "depth"] |
| 30507      | CoCreatePaper | Collaborative paper creation and publishing   | ["auth", "d":"subspace_op", "op":"co_create_paper", "sid", "paper_id", "user_ids", "quality"]     |

### Social Actions (CIP 06)

| Kind Value | Event Name    | Purpose/Scope                       | Key Tags Structure                                                                                |
| ---------- | ------------- | ----------------------------------- | ------------------------------------------------------------------------------------------------- |
| 30600      | Like          | Like any object (paper, post, etc.) | ["auth", "d":"subspace_op", "op":"like", "sid", "object_id", "user_id"]                           |
| 30601      | Collect       | Collect/favorite any object         | ["auth", "d":"subspace_op", "op":"collect", "sid", "object_id", "user_id"]                        |
| 30602      | Share         | Share any object to a platform      | ["auth", "d":"subspace_op", "op":"share", "sid", "object_id", "user_id", "platform", "clicks"]    |
| 30603      | Comment       | Comment on any object               | ["auth", "d":"subspace_op", "op":"comment", "sid", "object_id", "user_id", "parent", "content"]   |
| 30604      | Tag           | Tag/label any object                | ["auth", "d":"subspace_op", "op":"tag", "sid", "object_id", "tag"]                                |
| 30605      | Follow        | Follow any user or entity           | ["auth", "d":"subspace_op", "op":"follow", "sid", "user_id", "target_id"]                         |
| 30606      | Unfollow      | Unfollow any user or entity         | ["auth", "d":"subspace_op", "op":"unfollow", "sid", "user_id", "target_id"]                       |
| 30607      | Question      | Ask a question about any object     | ["auth", "d":"subspace_op", "op":"question", "sid", "object_id", "user_id", "content", "quality"] |
| 30608      | Room          | Represents a chat room              | ["auth", "d":"subspace_op", "op":"room", "sid", "name", "description", "members"]                 |
| 30609      | MessageInRoom | Represents a chat message in a room | ["auth", "d":"subspace_op", "op":"message", "sid", "room_id", "content", "reply_to", "mentions"]  |

### Community Actions (CIP 07)

| Kind Value | Event Name      | Purpose/Scope                   | Key Tags Structure                                                                                                |
| ---------- | --------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| 30700      | CommunityCreate | Create a community              | ["auth", "d":"subspace_op", "op":"community_create", "sid", "community_id", "name", "type"]                       |
| 30701      | CommunityInvite | Invite user to a community      | ["auth", "d":"subspace_op", "op":"community_invite", "sid", "community_id", "inviter_id", "invitee_id", "method"] |
| 30702      | ChannelCreate   | Create a channel in a community | ["auth", "d":"subspace_op", "op":"channel_create", "sid", "community_id", "channel_id", "name", "type"]           |
| 30703      | ChannelMessage  | Post a message in a channel     | ["auth", "d":"subspace_op", "op":"channel_message", "sid", "channel_id", "user_id", "content", "reply_to"]        |
