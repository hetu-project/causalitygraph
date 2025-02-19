# NIP-3041 - Poll & Vote Event with VLC

> Poll and vote events with VLC in a p2p network. Polls are directly integrated into message events kind 301(original kind 1), and responses are vote events kind 309(original kind 9). A modified version of Event Counts (NIP-45) is used to implement the query of VLC for a specific SID in relay.



### Specs

#### Kind `301` - Message & Poll

To create a poll, include a `poll` tag in the message event. with the following format:

```
tag: poll
options:
 - <multi|single> allow others to reply with one or multiple options
 - <clock> optional(not confirmed)LC(in depth|VLC) when surv expires,0 LC the uplimit of VLC(2M)
 - <start> poll start timestamp
 - <end> poll end timestamp
 - <title>event name
 - <info>optional Poll & Vote information ("" if not present)
 - [<option>]: array of string
```

**Example - Message**

```
{
  ...
  "kind": 301,
  "tags": [
    ...
    ["poll", "single", "0","1707294126","1707294126", "I'm a title!","This a demo survey!" "Option 1", "Option 2", "Option 3"],
    ...
  ],
}
```

The event ID of kind 301 is used to represent the SID of a specific state.

#### Kind `309` - Vote

To vote for a poll, you have to send an event kind `309`.\
The format of the event is the following:

```
kind: 309
content: <reason for voting> // Can be empty string
tag: poll_r // for poll response
options:
  - [<choices>]: array of index of the option | the first option is 0
```

**Example - Vote**

```
{
  ...
  "kind": 309,
  "tags": [
    ...
    ["e", "<event-id>"],
    ["poll_r", "0"],
    // OR
    ["poll_r", "0", "2"]
    ...
  ],
  "content": "reason for voting"
}
```


#### State Query

Relays implement Poll & Vote should to count the result of Vote, to simplify the design, the VLC will not be responsible for state-related calculations. The format of the State Query is the following:

```
["QUERY", <Specific SID>(Event ID)]
```

Results are returned using a RESULT response in the form:

```
[
    "Option 1",
     <integer>,
    "Option 2",
     <integer>,
    "Option 3",
    <integer>,
    ...
]
```

Whether Relay refuses to reply or times out, which is determined by the Relay or Client itself.

If client need full stat, it should download all kind 309 associated with Specific SID and calculate itself.



#### SIDs Query
Client sends message to relay to get the list of SIDs(Event ID)

```
["QUERYPOLLLIST",""]
```
Response from relay is:
```
[
  [
    "id": <Specific SID>,
    "title": <title>,
    "info": <info>
  ],
  [
    "id": <Specific SID>,
    "title": <title>,
    "info": <info>
  ],
  [
    "id": <Specific SID>,
    "title": <title>,
    "info": <info>
  ]
  ...
]
```

#### Query Poll metadata
Client sends message to relay to get the contents of poll through SIDs(Event ID)

```
["QUERYEVENTMETA",<Specific SID>]
```
Response from relay is:
```
{
    "id": <Specific SID>,
    "pubkey": <public key>,
    "created_at": <timestamp>,
    "kind": <kind code>,
    "tags": [
        {
	    "values": [
			"poll",
			"single",
			"0",
			"2024-02-21T08:37",
			"2024-02-21T20:37",
			"testdemo",
			"123456",
			"Yes",
			"No"
		]
	}
    ],
    "content": "",
    "sig": <signature>
}
```