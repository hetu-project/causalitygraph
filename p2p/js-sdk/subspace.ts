import crypto from 'crypto'

// Subspace event kinds
export const KindSubspaceCreate = 30100
export const KindSubspaceJoin = 30200
export const KindSubspaceOp = 30300

// Basic operation types (core operations)
export const OpPost = 'post' // 1
export const OpPropose = 'propose' // 2
export const OpVote = 'vote' // 3
export const OpInvite = 'invite' // 4

// Default operations string for subspace creation
export const DefaultSubspaceOps = 'post=1,propose=2,vote=3,invite=4'

// Tag type definition
export type Tag = string[]
export type Tags = Tag[]

// Nostr Event interface
export interface NostrEvent {
  id?: string
  pubkey?: string
  created_at: number
  kind: number
  tags: Tags
  content: string
  sig?: string
}

// SubspaceCreateEvent
export interface SubspaceCreateEvent {
  kind: number
  createdAt: number
  subspaceID: string
  subspaceName: string
  ops: string
  rules: string
  description: string
  imageURL: string
  tags: Tag[]
  content: string
}

// SubspaceJoinEvent
export interface SubspaceJoinEvent {
  kind: number
  createdAt: number
  subspaceID: string
  tags: Tag[]
  content: string
}

// SubspaceOpEvent
export interface SubspaceOpEvent {
  kind: number
  createdAt: number
  subspaceID: string
  operation: string
  tags: Tag[]
  content: string
  proposalID?: string
  vote?: string
  inviteePubkey?: string
  contributions?: string
  contentType?: string
  parentHash?: string
}

// Helper function to calculate SHA256 hash
function calculateSHA256(input: string): string {
  const hash = crypto.createHash('sha256').update(input).digest('hex')
  return `0x${hash}`
}

// Function to convert Subspace events to Nostr Event
export function toNostrEvent(event: SubspaceCreateEvent | SubspaceJoinEvent | SubspaceOpEvent): NostrEvent {
  const tags: Tags = event.tags.map(tag => tag)

  return {
    created_at: event.createdAt,
    kind: event.kind,
    tags,
    content: event.content,
  }
}

// Example usage for SubspaceCreateEvent
export function NewSubspaceCreateEvent(
  subspaceName: string,
  ops: string,
  rules: string,
  description: string,
  imageURL: string,
): SubspaceCreateEvent {
  const subspaceID = calculateSHA256(subspaceName + ops + rules)
  const tags: Tag[] = [
    ['d', 'subspace_create'],
    ['sid', subspaceID],
    ['subspace_name', subspaceName],
    ['ops', ops],
  ]
  if (rules) {
    tags.push(['rules', rules])
  }

  const content = JSON.stringify({
    desc: description,
    img_url: imageURL,
  })

  return {
    kind: KindSubspaceCreate,
    createdAt: Math.floor(Date.now() / 1000),
    subspaceID,
    subspaceName,
    ops,
    rules,
    description,
    imageURL,
    tags,
    content,
  }
}

// Example usage for SubspaceJoinEvent
export function NewSubspaceJoinEvent(subspaceID: string): SubspaceJoinEvent {
  const tags: Tag[] = [
    ['d', 'subspace_join'],
    ['sid', subspaceID],
  ]

  return {
    kind: KindSubspaceJoin,
    createdAt: Math.floor(Date.now() / 1000),
    subspaceID,
    tags,
    content: '*12345',
  }
}

// Example usage for SubspaceOpEvent
export function NewSubspaceOpEvent(
  subspaceID: string,
  operation: string,
  content: string,
  contentType?: string,
  parentHash?: string,
): SubspaceOpEvent {
  const tags: Tag[] = [
    ['d', 'subspace_op'],
    ['sid', subspaceID],
    ['ops', operation],
  ]
  if (contentType) {
    tags.push(['content_type', contentType])
  }
  if (parentHash) {
    tags.push(['parent', parentHash])
  }

  return {
    kind: KindSubspaceOp,
    createdAt: Math.floor(Date.now() / 1000),
    subspaceID,
    operation,
    contentType,
    parentHash,
    tags,
    content,
  }
}

// calculateSubspaceID generates a unique subspace ID based on subspaceName, ops, and rules
export function calculateSubspaceID(subspaceName: string, ops: string, rules: string): string {
  const input = subspaceName + ops + rules
  return calculateSHA256(input)
}

// ops Verification Logic
export function ValidateSubspaceCreateEvent(event: SubspaceCreateEvent): void {
  if (event.kind !== KindSubspaceCreate) {
    throw new Error(`Invalid event kind: expected ${KindSubspaceCreate}, got ${event.kind}`)
  }

  const requiredTags = ['d', 'sid', 'subspace_name', 'ops']
  const tagKeys = event.tags.map(tag => tag[0])

  for (const tag of requiredTags) {
    if (!tagKeys.includes(tag)) {
      throw new Error(`Missing required tag: ${tag}. Ensure the tag is included in the event.`)
    }
  }

  const calculatedSID = calculateSubspaceID(event.subspaceName, event.ops, event.rules)
  if (event.subspaceID !== calculatedSID) {
    throw new Error(`Invalid subspace ID: expected ${calculatedSID}, got ${event.subspaceID}`)
  }

  const content = JSON.parse(event.content)
  if (!content.desc) {
    throw new Error('Missing description in content')
  }

  const opsParts = event.ops.split(',')
  for (const part of opsParts) {
    const [key, value] = part.split('=')
    if (!key || !value || isNaN(Number(value))) {
      throw new Error(`Invalid ops format: ${event.ops}. Each part must be in the format key=value.`)
    }
  }
}

// ValidateSubspaceJoinEvent validates a SubspaceJoinEvent
export function ValidateSubspaceJoinEvent(event: SubspaceJoinEvent): void {
  if (event.kind !== KindSubspaceJoin) {
    throw new Error(`Invalid event kind: expected ${KindSubspaceJoin}, got ${event.kind}`)
  }

  const requiredTags = ['d', 'sid']
  const tagKeys = event.tags.map(tag => tag[0])

  for (const tag of requiredTags) {
    if (!tagKeys.includes(tag)) {
      throw new Error(`Missing required tag: ${tag}`)
    }
  }

  if (!event.subspaceID.startsWith('0x') || event.subspaceID.length !== 66) {
    throw new Error('Invalid subspace ID format or length')
  }
}

// SetContentType sets the content type for the operation
export function SetContentType(event: SubspaceOpEvent, contentType: string): void {
  event.contentType = contentType
  event.tags.push(['content_type', contentType])
}

// SetParent sets the parent event hash
export function SetParent(event: SubspaceOpEvent, parentHash: string): void {
  event.parentHash = parentHash
  event.tags.push(['parent', parentHash])
}

// SetProposal sets the proposal ID and rules
export function SetProposal(event: SubspaceOpEvent, proposalID: string, rules?: string): void {
  event.proposalID = proposalID
  event.tags.push(['proposal_id', proposalID])
  if (rules) {
    event.tags.push(['rules', rules])
  }
}

// SetVote sets the vote for a proposal
export function SetVote(event: SubspaceOpEvent, proposalID: string, vote: string): void {
  event.proposalID = proposalID
  event.vote = vote
  event.tags.push(['proposal_id', proposalID], ['vote', vote])
}

// SetInvite sets the invitee pubkey and rules
export function SetInvite(event: SubspaceOpEvent, inviteePubkey: string, rules?: string): void {
  event.inviteePubkey = inviteePubkey
  event.tags.push(['invitee_pubkey', inviteePubkey])
  if (rules) {
    event.tags.push([rules])
  }
}

// SetContributions sets the contribution weights
export function SetContributions(event: SubspaceOpEvent, contributions: string): void {
  event.contributions = contributions
  event.tags.push(['contrib', contributions])
}

// ValidateSubspaceOpEvent validates a SubspaceOpEvent
export function ValidateSubspaceOpEvent(event: SubspaceOpEvent): void {
  if (event.kind !== KindSubspaceOp) {
    throw new Error(`Invalid event kind: expected ${KindSubspaceOp}, got ${event.kind}`)
  }

  const requiredTags = ['d', 'sid', 'ops']
  const tagKeys = event.tags.map(tag => tag[0])

  for (const tag of requiredTags) {
    if (!tagKeys.includes(tag)) {
      throw new Error(`Missing required tag: ${tag}`)
    }
  }

  if (!event.subspaceID.startsWith('0x') || event.subspaceID.length !== 66) {
    throw new Error('Invalid subspace ID format or length')
  }

  switch (event.operation) {
    case OpPost:
      if (!event.contentType) {
        throw new Error('content_type is required for post operation')
      }
      break
    case OpPropose:
      if (!event.proposalID) {
        throw new Error('proposal_id is required for propose operation')
      }
      break
    case OpVote:
      if (!event.proposalID || !event.vote) {
        throw new Error('proposal_id and vote are required for vote operation')
      }
      if (event.vote !== 'yes' && event.vote !== 'no') {
        throw new Error(`Invalid vote value: ${event.vote}`)
      }
      break
    case OpInvite:
      if (!event.inviteePubkey) {
        throw new Error('invitee_pubkey is required for invite operation')
      }
      break
    default:
      throw new Error(`Unknown operation: ${event.operation}`)
  }
}

// ParseSubspaceOpEvent parses a raw event into a SubspaceOpEvent
export function ParseSubspaceOpEvent(rawEvent: NostrEvent): SubspaceOpEvent {
  const opEvent: SubspaceOpEvent = {
    kind: rawEvent.kind,
    createdAt: rawEvent.created_at,
    subspaceID: '',
    operation: '',
    tags: rawEvent.tags,
    content: rawEvent.content,
  }

  for (const tag of rawEvent.tags) {
    const [key, value] = tag
    switch (key) {
      case 'sid':
        opEvent.subspaceID = value
        break
      case 'ops':
        opEvent.operation = value
        break
      case 'content_type':
        opEvent.contentType = value
        break
      case 'parent':
        opEvent.parentHash = value
        break
      case 'proposal_id':
        opEvent.proposalID = value
        break
      case 'vote':
        opEvent.vote = value
        break
      case 'invitee_pubkey':
        opEvent.inviteePubkey = value
        break
      case 'contrib':
        opEvent.contributions = value
        break
    }
  }

  ValidateSubspaceOpEvent(opEvent)
  return opEvent
}
