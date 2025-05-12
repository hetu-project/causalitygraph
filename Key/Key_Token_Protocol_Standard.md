#  Key Token Protocol Standard v1 (Draft)

This document outlines a proposed standard for implementing tokens on the causality key.

## Core Functionality

*   **Issue Token:** Defines the initial creation and configuration of a token.
*   **Transfer:**  Specifies how tokens are transferred between accounts.
*   **Approve:**  Allows one account to authorize another to spend tokens on its behalf.
*   **MintCredit:**  Defines the process for creating new tokens based on predefined rules.
*   **Total Supply:** (RPC/RESTful API) Provides the total number of tokens in circulation.
*   **Balance Of:** (RPC/RESTful API) Returns the token balance of a specific account.
*   **Allowance:** (RPC/RESTful API)  Returns the amount of tokens that one account is allowed to spend on behalf of another.
*   **Symbol/Name/Decimals:** (RPC/RESTful API) Provides metadata about the token.

## Event Kinds

This standard utilizes specific event kinds to represent different token operations.

### Issue Token (Kind 30320)

This event is used to issue a new token.

```json
{
  "id": "...",
  "kind": 30320,
  "pubkey": "<issuer_pubkey>",
  "created_at": 1710000000,
  "tags": [
    ["symbol", "NOST"],
    ["name", "New Coin"],
    ["decimals", "6"]
  ],
  "content": "A decentralized token on Hetu.",
  "sig": "..."
}
```
symbol: The token symbol (e.g., "ca").

name: The token name (e.g., "new Coin").

decimals: The number of decimal places the token supports (e.g., "6").

### Transfer (Kind 30321)

This event represents a token transfer between two accounts.

```JSON
{
  "kind": 30321,
  "pubkey": "<sender_pubkey>",
  "created_at": 1710000123,
  "tags": [
    ["from", "<send_pubkey>"],
    ["to", "<receiver_pubkey>"],
    ["symbol", "NOST"],
    ["amount", "1000000"]
  ],
  "content": "Payment for something",
  "id": "...",
  "sig": "..."
}
```
from: The sender's public key.

to: The receiver's public key.

symbol: The token symbol.

amount: The amount of tokens being transferred (in the smallest unit).

### Approve (Kind 30322)

This event allows one account to authorize another to spend tokens on its behalf.

```JSON
{
  "kind": 30322,
  "pubkey": "<owner_pubkey>",
  "tags": [
    ["spender", "<spender_pubkey>"],
    ["symbol", "NOST"],
    ["amount", "5000000"]
  ],
  "content": "Approve DApp to use my NOST.",
  "sig": "..."
}
```
spender: The public key of the account being authorized.

symbol: The token symbol.

amount: The maximum amount of tokens the spender is allowed to spend (in the smallest unit).

### MintCredit (Kind 30323)

This event represents the creation of new tokens based on predefined rules. Two methods are supported: multiple events, each representing a single rule, or a single event embedding multiple rules.

Method 1: Multiple Events (Single Rule per Event)

```JSON
[
  {
    "kind": 30323,
    "pubkey": "<issuer_pubkey>",
    "created_at": 1712420000,
    "tags": [
      ["symbol", "NOST"],
      ["mint_if", "30023"],
      ["tag_key", "e"],
      ["tag_value", "<event_id_or_any>"],
      ["threshold", "10"],
      ["mint_amount", "1000000"]
    ],
    "content": "Active chat reward rule",
    "sig": "..."
  },
  {
    "kind": 30323,
    "pubkey": "<issuer_pubkey>",
    "tags": [
      ["symbol", "NOST"],
      ["mint_if", "30011"],
      ["tag_key", "p"],
      ["tag_value", "<some_pubkey>"],
      ["threshold", "3"],
      ["mint_amount", "500000"]
    ],
    "content": "Reward for liking a specific user 3 times",
    "sig": "..."
  }
]
```
Method 2: Single Event (Multiple Rules)

```JSON
{
  "kind": 30323,
  "tags": [
    ["rule", "symbol=NOST", "mint_if=30023", "tag_key=e", "tag_value=any", "threshold=10", "mint_amount=1000000"],
    ["rule", "symbol=NOST", "mint_if=30011", "tag_key=p", "tag_value=<some_pubkey>", "threshold=3", "mint_amount=500000"]
  ],
  "content": "Multi-rule MintCredit configuration",
  "sig": "..."
}
```
symbol: The token symbol.

mint_if: The event kind that triggers the minting process (e.g., "30023").

tag_key: The tag key to check in the triggering event (e.g., "e" for event reference).

tag_value: The specific tag value to match (can be a specific event ID or "any").

threshold: The minimum number of matching events required to trigger the MintCredit.

mint_amount: The amount of tokens to MintCredit (in the smallest unit).

### Query Interface
A rule parsing service will provide an interface for querying token information.

## Detailed Design

1. Event Sending Process
  - Client Sends Event to Relay: The client sends an EVENT message to the Relay using WebSocket.
  - Relay Validates and Deduplicates:

2. Checks event format validity.
  - Verifies the signature using the pubkey and sig fields.
  - Checks for duplicate events based on the event ID.

3. Assigns Logical Clock:
  - If the logical clock mechanism is enabled:
    - The relay increments its local clock.
    - The clock value is added to the event as a field or tag.

4. Forwards Event to Other Relays:
  - The event is forwarded to other relays using a gossip protocol or a configured list of relays.
  - Communication can be optimized using compression, signature encapsulation, or multiplexing.

5. Receiving Relay Processes Event:
  - The receiving relay again deduplicates and validates the event.
  - The logical clock is merged (e.g., Lamport Clock takes the max).
  - If the event already exists, the created_at and clock values are used to determine if an update is needed.

6. Stores and Re-broadcasts:
  - The event is stored in the local database.
  - The event is re-broadcast to locally connected clients that match the subscription filters.
