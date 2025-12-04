# Flux Protocol Standard v1 (Draft)

This document outlines a proposed standard for implementing Flux protocols on the causality key. Flux is an off-chain credit/scoring system that supports two main protocols: **FLUX20** (Power protocol with non-transferable, pure consumption mode) and **FLUX21** (Soulbound Token protocol for recording cumulative reputation).

## FLUX Protocol Family Overview

### FLUX20 - Power Protocol Characteristics

- **Non-Transferable:** Flux Power cannot be transferred between accounts
- **Non-Mintable:** Total supply is fixed; no additional minting allowed
- **Pure Consumption Mode:** Can only be burned/consumed
- **Use Cases:** Publishing tasks, submitting solutions, validation voting

**Economic Model Example:**

- Total Supply: 21 million
- Divisibility: Indivisible units
- Consumption: One-way burn only

### FLUX21 - Soulbound Token (SBT) Protocol Characteristics

- **Credit SBT:** Accumulates contribution reputation
- **Bond SBT:** Service staking evidence
- **Non-Transferable:** Cannot be transferred between accounts
- **Updatable:** Can be updated to reflect new achievements
- **Use Cases:** Reputation tracking, staking verification, credential management

## Nostr Event Structure Foundation

All Flux events inherit from the base Nostr event structure defined in CausalityKey (NIP-01 compatible):

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": "<integer between 0 and 65535>",
  "tags": [
    ["auth", "action=<mask>", "key=<key-Id>", "exp=<expiration_clock>"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "<operation_name>"],
    ["parent", "<parent_event_id>"],
    ["<tag_key>", "<tag_value>"]
  ],
  "content": "<arbitrary string>",
  "sig": "<64 bytes lowercase hex-encoded ETH signature>"
}
```

**Common Fields:**

- `auth`: Permission declaration with action (mask: 1=read, 2=write, 4=execute), key (causality key ID), and exp (expiration clock)
- `d`: "subspace_op" - Indicates this is a subspace operation
- `sid`: Subspace ID where this Flux operation occurs
- `op`: Operation name (flux20_issue, flux20_allocation, flux20_burn, flux20_transfer, flux21_credit, flux21_bond, flux21_update)
- `parent`: Optional reference to parent event hash

## Event Kinds

This standard utilizes specific event kinds to represent different Flux operations.

### 5.1 FLUX20 Power Protocol (Non-Transferable, Purely Consumable)

#### 5.1.1 Power Issuance Event (Kind 30320)

This event creates and configures a new Flux Power with fixed supply, non-transferable, and burn-only characteristics.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the issuer>",
  "created_at": 1710000000,
  "kind": 30320,
  "tags": [
    ["auth", "action=2", "key=30320", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux20_issue"],
    ["symbol", "POWER"],
    ["name", "Flux Power"],
    ["total_supply", "21000000"],
    ["decimals", "0"],
    ["transferable", "false"],
    ["mintable", "false"]
  ],
  "content": "Flux Power - non-transferable, burn-only utility score for task publishing, solution submission, and validation voting",
  "sig": "<ETH signature>"
}
```

**Field Description:**

- `d`: "subspace_op" - Indicates this is a subspace operation
- `sid`: Subspace ID where this Flux Power operates
- `op`: "flux20_issue" - Operation identifier for power issuance
- `symbol`: Token symbol (e.g., "POWER")
- `name`: Human-readable name (e.g., "Flux Power")
- `total_supply`: Fixed supply (e.g., "21000000")
- `decimals`: Decimal places, typically "0" for indivisible units
- `transferable`: "false" - Power cannot be transferred between accounts
- `mintable`: "false" - No additional minting allowed after issuance

#### 5.1.2 Power Allocation Event (Kind 30321)

This event allocates Power to recipients. Allocation is a one-way assignment from issuer to recipient and cannot be transferred further.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the issuer>",
  "created_at": 1710000001,
  "kind": 30321,
  "tags": [
    ["auth", "action=2", "key=30321", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux20_allocation"],
    ["symbol", "POWER"],
    ["p", "<recipient_pubkey>"],
    ["amount", "10000"],
    ["purpose", "initial_allocation|reward|contribution"],
    ["vesting", "<optional_vesting_schedule>"]
  ],
  "content": "Allocate Flux Power to contributor or participant",
  "sig": "<ETH signature>"
}
```

**Field Description:**

- `d`: "subspace_op" - Indicates this is a subspace operation
- `sid`: Subspace ID
- `op`: "flux20_allocation" - Operation identifier for power allocation
- `symbol`: Token symbol (must match issuance)
- `p`: Recipient's ETH public key
- `amount`: Allocation amount (in smallest unit)
- `purpose`: Reason for allocation (initial_allocation, reward, or contribution)
- `vesting`: Optional vesting schedule or release conditions

#### 5.1.3 Power Burn Event (Kind 30322)

This event represents the consumption/burning of Power. Once burned, Power is permanently removed from circulation.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the burner>",
  "created_at": 1710000002,
  "kind": 30322,
  "tags": [
    ["auth", "action=2", "key=30322", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux20_burn"],
    ["symbol", "POWER"],
    ["amount", "100"],
    ["action", "task_publish|solution_submit|validation_vote|other"],
    ["ref", "<referenced_event_id>"],
    ["balance", "<remaining_balance>"]
  ],
  "content": "Burn Flux Power for performing an action",
  "sig": "<ETH signature>"
}
```

**Field Description:**

- `d`: "subspace_op" - Indicates this is a subspace operation
- `sid`: Subspace ID
- `op`: "flux20_burn" - Operation identifier for power burn
- `symbol`: Token symbol
- `amount`: Amount of Power to burn
- `action`: Type of action being performed (task_publish, solution_submit, validation_vote, etc.)
- `ref`: Reference to the event associated with this action
- `balance`: Remaining Power balance after burn (for tracking purposes)

#### 5.1.4 Flux20 Transfer Event (Kind 30323)

This event represents the transfer of Flux20 between accounts. Only applicable when Flux20 is marked as transferable.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the sender>",
  "created_at": 1710000003,
  "kind": 30323,
  "tags": [
    ["auth", "action=2", "key=30323", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux20_transfer"],
    ["symbol", "FLUX"],
    ["from", "<sender_pubkey>"],
    ["to", "<recipient_pubkey>"],
    ["amount", "1000"],
    ["to_sid", "<target_subspace_id>"],
    ["bridge_ref", "<bridge_event_or_proof>"]
  ],
  "content": "Transfer Flux to another account",
  "sig": "<ETH signature>"
}
```

**Field Description:**

- `d`: "subspace_op" - Indicates this is a subspace operation
- `sid`: Subspace ID
- `op`: "flux20_transfer" - Operation identifier for power transfer
- `symbol`: Token symbol
- `from`: Sender's ETH public key
- `to`: Recipient's ETH public key
- `amount`: Amount of Power to transfer

**Cross-subspace fields (optional):**

- `to_sid`: Target subspace ID when performing a cross-subspace transfer. If empty, transfer is inside the same subspace.
- `bridge_ref`: Reference (event id or proof) to an off-chain bridge/relayer operation that completes the cross-subspace transfer. Relayers should provide this proof when finalizing the transfer.

### 5.2 FLUX21 Soulbound Token (SBT) Protocol (Non-Transferable Reputation)

#### 5.2.1 Credit SBT Issuance Event (Kind 30330)

This event issues a Credit SBT that records cumulative contribution reputation to an individual recipient.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the issuer>",
  "created_at": 1710000004,
  "kind": 30330,
  "tags": [
    ["auth", "action=2", "key=30330", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux21_credit"],
    ["p", "<recipient_pubkey>"],
    ["symbol", "CREDIT"],
    ["name", "Flux Credit"],
    ["score", "850"],
    ["level", "novice|intermediate|expert|master"],
    ["category", "technical|social|governance|contribution"],
    ["transferable", "false"]
  ],
  "content": "{\"description\": \"Cumulative contribution credit record\", \"achievements\": [\"achievement1\", \"achievement2\"], \"issued_at\": \"<timestamp>\"}",
  "sig": "<ETH signature>"
}
```

**Field Description:**

- `d`: "subspace_op" - Indicates this is a subspace operation
- `sid`: Subspace ID
- `op`: "flux21_credit" - Operation identifier for credit SBT issuance
- `p`: Recipient's ETH public key
- `symbol`: "CREDIT" - Identifies this as a Credit SBT
- `name`: Human-readable name (e.g., "Flux Credit")
- `score`: Credit score value (e.g., "850")
- `level`: Reputation level (novice, intermediate, expert, master)
- `category`: Category of achievement (technical, social, governance, contribution)
- `transferable`: "false" - SBT cannot be transferred
- `content`: JSON structure containing description, list of achievements, and timestamp

#### 5.2.2 Bond SBT Issuance Event (Kind 30331)

This event issues a Bond SBT that records service staking and commitment evidence.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the issuer>",
  "created_at": 1710000005,
  "kind": 30331,
  "tags": [
    ["auth", "action=2", "key=30331", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux21_bond"],
    ["p", "<recipient_pubkey>"],
    ["symbol", "BOND"],
    ["name", "Flux Bond"],
    ["amount", "5000"],
    ["purpose", "validation_stake|service_guarantee|quality_assurance|other"],
    ["start_time", "1710000005"],
    ["expiry", "1712600000"],
    ["penalty", "<slashing_conditions>"],
    ["transferable", "false"]
  ],
  "content": "{\"terms\": \"Bond terms and conditions\", \"rights\": \"Bondholder rights and privileges\", \"conditions\": \"Release or penalty conditions\"}",
  "sig": "<ETH signature>"
}
```

**Field Description:**

- `d`: "subspace_op" - Indicates this is a subspace operation
- `sid`: Subspace ID
- `op`: "flux21_bond" - Operation identifier for bond SBT issuance
- `p`: Recipient's ETH public key (bond holder)
- `symbol`: "BOND" - Identifies this as a Bond SBT
- `name`: Human-readable name (e.g., "Flux Bond")
- `amount`: Bond amount/value staked
- `purpose`: Purpose of bonding (validation_stake, service_guarantee, quality_assurance, etc.)
- `start_time`: Unix timestamp when bond becomes active
- `expiry`: Unix timestamp when bond expires
- `penalty`: Conditions under which bondholder may face slashing or penalties
- `transferable`: "false" - SBT cannot be transferred
- `content`: JSON structure containing terms, rights, and release conditions

#### 5.2.3 SBT Update Event (Kind 30332)

This event updates an existing Credit or Bond SBT to reflect new achievements, score changes, or additional staking.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the updater>",
  "created_at": 1710000006,
  "kind": 30332,
  "tags": [
    ["auth", "action=2", "key=30332", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux21_update"],
    ["e", "<previous_sbt_event_id>"],
    ["p", "<sbt_owner_pubkey>"],
    ["symbol", "CREDIT|BOND"],
    ["new_score", "860"],
    ["new_amount", "5500"],
    ["new_level", "expert"],
    [
      "reason",
      "performance_update|additional_achievement|additional_stake|expiry_renewal|other"
    ],
    ["evidence", "<event_id_or_hash>"]
  ],
  "content": "{\"update_reason\": \"Completed high-quality task\", \"evidence_summary\": \"Summary of evidence\", \"updated_at\": \"<timestamp>\"}",
  "sig": "<ETH signature>"
}
```

**Field Description:**

- `d`: "subspace_op" - Indicates this is a subspace operation
- `sid`: Subspace ID
- `op`: "flux21_update" - Operation identifier for SBT updates
- `e`: Event ID of the previous SBT being updated
- `p`: SBT owner's ETH public key
- `symbol`: Type of SBT (CREDIT or BOND)
- `new_score`: Updated score value (for Credit SBT)
- `new_amount`: Updated amount (for Bond SBT)
- `new_level`: Updated level/tier (optional)
- `reason`: Reason for update
- `evidence`: Reference to evidence event or transaction hash
- `content`: JSON structure containing update reason, evidence summary, and timestamp

#### 5.1.5 Flux20 Withdraw Event (Kind 30324)

This event represents a withdrawal request to convert transferable Flux20 balance into an external on-chain ERC20 (or other) representation. It follows the same `subspace_op` structure and can be used to request, track and finalize asset exits (internal -> external).

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the requester>",
  "created_at": 1710000007,
  "kind": 30324,
  "tags": [
    ["auth", "action=2", "key=30324", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "<subspace_id>"],
    ["op", "flux20_withdraw"],
    ["symbol", "Flux"],
    ["p", "<owner_pubkey>"],
    ["amount", "1000"],
    ["to_chain", "ethereum|arbitrum|other"],
    ["to_address", "<external_recipient_address>"],
    ["fee", "<fee_amount>"],
    ["ref", "<withdraw_request_id>"],
    ["status", "requested|pending|completed|failed"],
    ["proof", "<onchain_tx_hash_or_receipt>"]
  ],
  "content": "Withdraw request: convert transferable Flux to external ERC20 on target chain",
  "sig": "<ETH signature>"
}
```

**Field Description:**

- `op`: "flux20_withdraw" - Operation identifier for withdrawal requests
- `to_chain`: Target external chain or system to receive the exchanged asset
- `to_address`: External address on the target chain to receive the tokens
- `fee`: Withdraw fee (if applicable)
- `ref`: Local withdraw request id for correlation
- `status`: Withdraw request lifecycle state
- `proof`: When completed, on-chain tx hash or receipt proving mint/transfer on external chain

### Query Interface

A rule parsing service will provide interfaces for querying Flux information:

- **Flux Balance:** Query current Flux Power balance for an account
- **Flux History:** Retrieve allocation and burn history
- **Reputation Score:** Query current Credit SBT score and level
- **Bond Status:** Query active bonds, expiry dates, and penalty conditions
- **SBT Metadata:** Retrieve full SBT details including achievements and evidence

## Example Scenarios

### Scenario 1: Power Issuance and Allocation

Alice creates a new Flux Power in a subspace with 21 million units:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash>",
  "pubkey": "<Alice's ETH address>",
  "created_at": 1710000000,
  "kind": 30320,
  "tags": [
    ["auth", "action=2", "key=30320", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "flux20_issue"],
    ["symbol", "POWER"],
    ["name", "Flux Power"],
    ["total_supply", "21000000"],
    ["decimals", "0"],
    ["transferable", "false"],
    ["mintable", "false"]
  ],
  "content": "Flux Power for task publishing and validation",
  "sig": "<Alice's ETH signature>"
}
```

Then Alice allocates Power to Bob:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash>",
  "pubkey": "<Alice's ETH address>",
  "created_at": 1710000001,
  "kind": 30321,
  "tags": [
    ["auth", "action=2", "key=30321", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "flux20_allocation"],
    ["symbol", "POWER"],
    ["p", "<Bob's ETH public key>"],
    ["amount", "10000"],
    ["purpose", "initial_allocation"],
    ["vesting", "linear_release_over_365_days"]
  ],
  "content": "Initial allocation of Flux Power to Bob for collaboration",
  "sig": "<Alice's ETH signature>"
}
```

### Scenario 2: Power Consumption

Bob burns some Power to publish a task:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash>",
  "pubkey": "<Bob's ETH address>",
  "created_at": 1710000010,
  "kind": 30322,
  "tags": [
    ["auth", "action=2", "key=30322", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "flux20_burn"],
    ["symbol", "POWER"],
    ["amount", "100"],
    ["action", "task_publish"],
    ["ref", "<task_event_id>"],
    ["balance", "9900"]
  ],
  "content": "Burn 100 POWER for publishing a new task",
  "sig": "<Bob's ETH signature>"
}
```

### Scenario 3: Credit SBT Issuance

Alice issues a Credit SBT to Bob for completing high-quality work:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash>",
  "pubkey": "<Alice's ETH address>",
  "created_at": 1710000020,
  "kind": 30330,
  "tags": [
    ["auth", "action=2", "key=30330", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "flux21_credit"],
    ["p", "<Bob's ETH public key>"],
    ["symbol", "CREDIT"],
    ["name", "Flux Credit"],
    ["score", "750"],
    ["level", "intermediate"],
    ["category", "technical"],
    ["transferable", "false"]
  ],
  "content": "{\"description\": \"Technical contribution credit for completing model validation\", \"achievements\": [\"completed_validation_task_01\", \"high_quality_submission\"], \"issued_at\": \"2024-01-01T00:00:00Z\"}",
  "sig": "<Alice's ETH signature>"
}
```

### Scenario 4: Bond SBT for Validation Stake

Alice issues a Bond SBT to Charlie for staking validation:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash>",
  "pubkey": "<Alice's ETH address>",
  "created_at": 1710000030,
  "kind": 30331,
  "tags": [
    ["auth", "action=2", "key=30331", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "flux21_bond"],
    ["p", "<Charlie's ETH public key>"],
    ["symbol", "BOND"],
    ["name", "Flux Validation Bond"],
    ["amount", "5000"],
    ["purpose", "validation_stake"],
    ["start_time", "1710000030"],
    ["expiry", "1742500030"],
    ["penalty", "10_percent_slashing_for_malicious_validation"],
    ["transferable", "false"]
  ],
  "content": "{\"terms\": \"Validator agrees to provide honest validation for 1 year\", \"rights\": \"Validator earns validation rewards\", \"conditions\": \"Bond can be slashed if malicious behavior detected\"}",
  "sig": "<Alice's ETH signature>"
}
```

### Scenario 5: Credit SBT Update

Alice updates Bob's Credit SBT score after completing additional work:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash>",
  "pubkey": "<Alice's ETH address>",
  "created_at": 1710000040,
  "kind": 30332,
  "tags": [
    ["auth", "action=2", "key=30332", "exp=0"],
    ["d", "subspace_op"],
    ["sid", "0xMG"],
    ["op", "flux21_update"],
    ["e", "<previous_credit_sbt_event_id>"],
    ["p", "<Bob's ETH public key>"],
    ["symbol", "CREDIT"],
    ["new_score", "850"],
    ["new_level", "expert"],
    ["reason", "additional_achievement"],
    ["evidence", "<task_completion_event_id>"]
  ],
  "content": "{\"update_reason\": \"Bob completed 3 additional high-impact technical tasks and demonstrated expertise\", \"evidence_summary\": \"Peer reviews and task completion metrics validate advancement to expert level\", \"updated_at\": \"2024-02-01T00:00:00Z\"}",
  "sig": "<Alice's ETH signature>"
}
```
