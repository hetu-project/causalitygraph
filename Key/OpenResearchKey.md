# OpenResearch Causality Key Events
## 1. Description

This proposal extends the Causality Key protocol to support the openResearch platform, which provides an integrated "search → analysis → collaboration → discovery" workflow for researchers and teams. By leveraging the Verifiable Logic Clock (VLC) within the Nostr event structure, openResearch enables decentralized tracking of research activities, paper annotations, collaborative analysis, and knowledge graph construction. User identities are bound to **ETH public key addresses** and all research-related events are signed using ETH signatures to ensure authenticity and provenance tracking in scientific discovery.

## 2. Key Features

- **Verifiable Paper Provenance**: Track paper submissions, annotations, and research discussions in a verifiable manner using VLC.
- **Collaborative Research Spaces**: Create and manage research-specific subspaces with fine-grained access control.
- **AI-Enhanced Research Operations**: Integrate AI capabilities with specific event types for paper analysis, summarization, and knowledge extraction.
- **Research Knowledge Graph**: Build and maintain knowledge graphs that connect papers, researchers, annotations, and discoveries.
- **Structured Annotation System**: Provide a comprehensive annotation framework with citation tracking and collaborative feedback.
- **Research Task Management**: Coordinate research activities with task assignment and progress tracking.

---

## 3. openResearch Subspace

The openResearch platform extends the basic Causality Key system with new kinds of events specific to research workflows:

### 3.1 Subspace Creation Event (Kind 30100)

Creating a research subspace with specialized operation types:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30100,
  "tags": [
    ["d", "subspace_create"],
    ["sid", "0xOR"],
    ["subspace_name", "openresearch"],
    ["ops", "paper=30501,annotation=30502,review=30503,task=30504,graph=30505,ai_analysis=30506,discussion=30507"],
    ["rules", "energy>500"]
  ],
  "content": "{\"desc\":\"Collaborative research space for literature analysis and discovery\", \"img_url\": \"http://image_addr.png\"}",
  "sig": "<ETH signature>"
}
```

### 3.2 OpenResearch Specific Event Types

| Kind Value | Event Name | Purpose | Key Tags Structure |
| --- | --- | --- | --- |
| 30501 | Paper | Submit or index a research paper | ["auth", "d":"subspace_op", "sid", "doi", "paper_type", "authors"] |
| 30502 | Annotation | Create annotation on paper text | ["auth", "d":"subspace_op", "sid", "paper_id", "position", "type"] |
| 30503 | Review | Submit structured review of a paper | ["auth", "d":"subspace_op", "sid", "paper_id", "rating", "aspects"] |
| 30504 | Task | Create or update research tasks | ["auth", "d":"subspace_op", "sid", "task_type", "assignee", "deadline"] |
| 30505 | Graph | Add or update knowledge graph elements | ["auth", "d":"subspace_op", "sid", "node_type", "edge_type", "references"] |
| 30506 | AI_Analysis | Request or submit AI analysis results | ["auth", "d":"subspace_op", "sid", "analysis_type", "paper_ids", "prompt"] |
| 30507 | Discussion | Create or contribute to research discussions | ["auth", "d":"subspace_op", "sid", "topic", "parent", "references"] |

## 4. Operation Event Details

### 4.1 Paper Event (Kind 30501)

Used to submit, index, or reference a research paper in the system.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30501,
  "tags": [
    ["auth", "action=3", "key=30501", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xOR"],
    ["op", "paper"],
    ["doi", "10.xxxx/xxxxx"],
    ["paper_type", "article"],
    ["authors", "Author1,Author2,Author3"],
    ["keywords", "keyword1,keyword2,keyword3"],
    ["year", "2023"],
    ["journal", "Journal Name"]
  ],
  "content": "{\"title\":\"Paper Title\",\"abstract\":\"Paper abstract text...\",\"url\":\"https://doi.org/10.xxxx/xxxxx\",\"file_hash\":\"ipfs://bafy...\"}",
  "sig": "<ETH signature>"
}
```

### 4.2 Annotation Event (Kind 30502)

Used to create annotations on specific sections of papers.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30502,
  "tags": [
    ["auth", "action=2", "key=30502", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xOR"],
    ["op", "annotation"],
    ["paper_id", "<paper_event_id>"],
    ["position", "section:2,paragraph:3,offset:120,length:250"],
    ["type", "comment"],
    ["parent", "<parent_annotation_id>"]
  ],
  "content": "This finding contradicts the results from Smith et al. (2022), which might be due to different experimental conditions.",
  "sig": "<ETH signature>"
}
```

### 4.3 Review Event (Kind 30503)

Used for structured reviews of research papers.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30503,
  "tags": [
    ["auth", "action=2", "key=30503", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xOR"],
    ["op", "review"],
    ["paper_id", "<paper_event_id>"],
    ["rating", "4.5"],
    ["aspects", "methodology:4,novelty:5,clarity:4,reproducibility:3"]
  ],
  "content": "{\"summary\":\"This paper presents a novel approach to...\",\"strengths\":\"The methodology is robust and...\",\"weaknesses\":\"The results section lacks...\",\"recommendations\":\"Authors should consider...\"}",
  "sig": "<ETH signature>"
}
```

### 4.4 Task Event (Kind 30504)

Used for research task management within collaborative spaces.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30504,
  "tags": [
    ["auth", "action=3", "key=30504", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xOR"],
    ["op", "task"],
    ["task_type", "literature_review"],
    ["assignee", "<assignee_pubkey>"],
    ["deadline", "<unix_timestamp>"],
    ["status", "in_progress"],
    ["priority", "high"],
    ["related_papers", "<paper_id1>,<paper_id2>"]
  ],
  "content": "{\"title\":\"Review recent advances in quantum computing algorithms\",\"description\":\"Compile a comprehensive review of papers published in the last 2 years on quantum computing algorithms for optimization problems.\",\"deliverables\":\"Summary report with comparative analysis\"}",
  "sig": "<ETH signature>"
}
```

### 4.5 Graph Event (Kind 30505)

Used to build and maintain the research knowledge graph.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30505,
  "tags": [
    ["auth", "action=2", "key=30505", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xOR"],
    ["op", "graph"],
    ["node_type", "concept"],
    ["edge_type", "relates_to"],
    ["references", "<paper_id1>,<paper_id2>"]
  ],
  "content": "{\"source_node\":\"quantum_entanglement\",\"target_node\":\"quantum_teleportation\",\"relationship\":\"enables\",\"weight\":0.85,\"description\":\"Quantum entanglement is a fundamental prerequisite for quantum teleportation protocols.\"}",
  "sig": "<ETH signature>"
}
```

### 4.6 AI_Analysis Event (Kind 30506)

Used to request and store AI-generated analysis of research papers.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30506,
  "tags": [
    ["auth", "action=3", "key=30506", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xOR"],
    ["op", "ai_analysis"],
    ["analysis_type", "literature_gap"],
    ["paper_ids", "<paper_id1>,<paper_id2>,<paper_id3>"],
    ["prompt", "Identify research gaps and potential future directions in these papers"]
  ],
  "content": "{\"analysis_result\":\"Based on the provided papers, several research gaps emerge: 1) Limited exploration of [specific area]...\",\"key_insights\":[\"insight1\",\"insight2\"],\"potential_directions\":[\"direction1\",\"direction2\"]}",
  "sig": "<ETH signature>"
}
```

### 4.7 Discussion Event (Kind 30507)

Used for threaded discussions about research topics.

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30507,
  "tags": [
    ["auth", "action=2", "key=30507", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xOR"],
    ["op", "discussion"],
    ["topic", "quantum_computing_ethics"],
    ["parent", "<parent_discussion_id>"],
    ["references", "<paper_id1>,<paper_id2>"]
  ],
  "content": "The ethical implications of quantum computing for cryptography should be addressed proactively. As discussed in Smith et al. (2022), quantum computers could potentially break current encryption standards.",
  "sig": "<ETH signature>"
}
```

## 5. Integration with Governance and Existing Subspaces

OpenResearch can be integrated with existing governance mechanisms defined in the base Causality Key system:

### 5.1 Research Proposal Voting

Utilizing the governance subspace (30300-30304) for research-related proposals:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30301,
  "tags": [
    ["auth", "action=2", "key=30301", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xOR"],
    ["op", "propose"],
    ["proposal_id", "research_proposal_001"],
    ["related_research", "<paper_id1>,<paper_id2>"],
    ["rules", "min_papers>5,min_reviews>3"]
  ],
  "content": "Proposal to establish a new research focus area on quantum computing ethics within our collaborative space",
  "sig": "<ETH signature>"
}
```

### 5.2 Research Credit Allocation

Utilizing token operations for research contributions:

```json
{
  "id": "<32 bytes lowercase hex-encoded sha256 hash of the serialized event data>",
  "pubkey": "<32 bytes lowercase hex-encoded ETH address of the event creator>",
  "created_at": "<Unix timestamp in seconds>",
  "kind": 30304,
  "tags": [
    ["auth", "action=3", "key=30304", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xOR"],
    ["op", "mint"],
    ["token_name", "Research Credit"],
    ["token_symbol", "RCRED"],
    ["token_decimals", "18"],
    ["initial_supply", "1000000"],
    ["drop_ratio", "30501:5,30502:1,30503:3,30504:2,30505:4,30506:3,30507:1"]
  ],
  "content": "Research credit token for rewarding contributions to collaborative research",
  "sig": "<ETH signature>"
}
```

## 6. Complete Application Flow Examples

### 6.1 Creating a Literature Review Workflow

```json
// 1. Create research subspace
{
  "id": "<hash>",
  "pubkey": "<researcher_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30100,
  "tags": [
    ["d", "subspace_create"],
    ["sid", "0xQC"],
    ["subspace_name", "quantum_computing_review"],
    ["ops", "paper=30501,annotation=30502,review=30503,task=30504,graph=30505,ai_analysis=30506,discussion=30507"],
    ["rules", "energy>300"]
  ],
  "content": "{\"desc\":\"Collaborative literature review on quantum computing advances\"}",
  "sig": "<signature>"
}

// 2. Submit a paper to the review group
{
  "id": "<hash>",
  "pubkey": "<researcher_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30501,
  "tags": [
    ["auth", "action=3", "key=30501", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xQC"],
    ["op", "paper"],
    ["doi", "10.xxxx/quantum.2023.01"],
    ["paper_type", "article"],
    ["authors", "Quantum Researcher et al."]
  ],
  "content": "{\"title\":\"Recent Advances in Quantum Algorithms\",\"abstract\":\"...\"}",
  "sig": "<signature>"
}

// 3. Create annotations on paper
{
  "id": "<hash>",
  "pubkey": "<reviewer_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30502,
  "tags": [
    ["auth", "action=2", "key=30502", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xQC"],
    ["op", "annotation"],
    ["paper_id", "<paper_hash>"],
    ["position", "section:methodology,paragraph:2"]
  ],
  "content": "The experimental setup needs more clarity regarding quantum decoherence controls.",
  "sig": "<signature>"
}

// 4. Request AI analysis
{
  "id": "<hash>",
  "pubkey": "<researcher_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30506,
  "tags": [
    ["auth", "action=3", "key=30506", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xQC"],
    ["op", "ai_analysis"],
    ["analysis_type", "comparison"],
    ["paper_ids", "<paper_hash1>,<paper_hash2>,<paper_hash3>"]
  ],
  "content": "{\"prompt\":\"Compare methodologies and results of these quantum computing papers\"}",
  "sig": "<signature>"
}

// 5. Create knowledge graph connection
{
  "id": "<hash>",
  "pubkey": "<researcher_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30505,
  "tags": [
    ["auth", "action=2", "key=30505", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xQC"],
    ["op", "graph"],
    ["node_type", "algorithm"],
    ["edge_type", "improves_upon"],
    ["references", "<paper_hash1>,<paper_hash2>"]
  ],
  "content": "{\"source_node\":\"Grover's Algorithm\",\"target_node\":\"Modified Grover's Algorithm\",\"improvement\":\"15% speed increase\"}",
  "sig": "<signature>"
}
```

### 6.2 Research Collaboration Task Management

```json
// 1. Create research task
{
  "id": "<hash>",
  "pubkey": "<project_lead_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30504,
  "tags": [
    ["auth", "action=3", "key=30504", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xQC"],
    ["op", "task"],
    ["task_type", "experiment_replication"],
    ["assignee", "<researcher_pubkey>"],
    ["deadline", "<timestamp+14days>"],
    ["priority", "high"]
  ],
  "content": "{\"title\":\"Replicate quantum interference pattern experiment\",\"description\":\"...\"}",
  "sig": "<signature>"
}

// 2. Update task status
{
  "id": "<hash>",
  "pubkey": "<researcher_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30504,
  "tags": [
    ["auth", "action=2", "key=30504", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xQC"],
    ["op", "task"],
    ["task_id", "<original_task_hash>"],
    ["status", "completed"],
    ["related_papers", "<new_paper_hash>"]
  ],
  "content": "{\"completion_notes\":\"Successfully replicated the experiment with 98% confidence interval\",\"data_url\":\"ipfs://bafy...\"}",
  "sig": "<signature>"
}

// 3. Reward research contributions
{
  "id": "<hash>",
  "pubkey": "<project_lead_pubkey>",
  "created_at": "<timestamp>",
  "kind": 30321,
  "tags": [
    ["auth", "action=3", "key=30321", "exp=500000"],
    ["d", "subspace_op"],
    ["sid", "0xQC"],
    ["op", "transfer"],
    ["from", "<project_fund_pubkey>"],
    ["to", "<researcher_pubkey>"],
    ["symbol", "RCRED"],
    ["amount", "500"]
  ],
  "content": "Research credit reward for successful experiment replication",
  "sig": "<signature>"
}
``` 