import { expect, test } from 'bun:test'
import {
  KindSubspaceCreate,
  KindSubspaceJoin,
  KindSubspaceOp,
  OpPost,
  OpPropose,
  OpVote,
  OpInvite,
  NostrEvent,
  NewSubspaceCreateEvent,
  ValidateSubspaceCreateEvent,
  NewSubspaceJoinEvent,
  ValidateSubspaceJoinEvent,
  NewSubspaceOpEvent,
  SetContentType,
  SetParent,
  SetProposal,
  SetVote,
  SetInvite,
  SetContributions,
  ValidateSubspaceOpEvent,
  ParseSubspaceOpEvent,
} from './subspace.ts'

test('SubspaceCreateEvent: should create a valid event', () => {
  const event = NewSubspaceCreateEvent(
    'TestSubspace',
    'post=1,propose=2',
    'rule1',
    'This is a test subspace',
    'http://example.com/image.png',
  )

  expect(event.kind).toBe(KindSubspaceCreate)
  expect(event.subspaceName).toBe('TestSubspace')
  expect(event.ops).toBe('post=1,propose=2')
  expect(event.rules).toBe('rule1')
  expect(event.description).toBe('This is a test subspace')
  expect(event.imageURL).toBe('http://example.com/image.png')
  expect(event.tags).toContainEqual(['d', 'subspace_create']) 
  expect(event.tags).toContainEqual(['sid', event.subspaceID]) 
})

test('SubspaceCreateEvent: should validate a valid event', () => {
  const event = NewSubspaceCreateEvent(
    'TestSubspace',
    'post=1,propose=2',
    'rule1',
    'This is a test subspace',
    'http://example.com/image.png',
  )

  expect(() => ValidateSubspaceCreateEvent(event)).not.toThrow()
})

test('SubspaceCreateEvent: should throw error for invalid ops', () => {
  const event = NewSubspaceCreateEvent('TestSubspace', 'invalid_ops', '', 'TestSubspace invalid_ops', '')

  expect(() => ValidateSubspaceCreateEvent(event)).toThrow(
    'Invalid ops format: invalid_ops. Each part must be in the format key=value.',
  )
})

test('SubspaceJoinEvent: should create a valid event', () => {
  const event = NewSubspaceJoinEvent('0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef')

  expect(event.kind).toBe(KindSubspaceJoin)
  expect(event.subspaceID).toBe('0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef')
  expect(event.tags).toContainEqual(['d', 'subspace_join']) 
  expect(event.tags).toContainEqual(['sid', '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef']) 
})

test('SubspaceJoinEvent: should validate a valid event', () => {
  const event = NewSubspaceJoinEvent('0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef')

  expect(() => ValidateSubspaceJoinEvent(event)).not.toThrow()
})

test('SubspaceJoinEvent: should throw error for invalid subspace ID', () => {
  const event = NewSubspaceJoinEvent('invalid_sid')

  expect(() => ValidateSubspaceJoinEvent(event)).toThrow('Invalid subspace ID format or length')
})

test('SubspaceOpEvent: should create a valid event', () => {
  const event = NewSubspaceOpEvent(
    '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
    OpPost,
    'Test content',
  )

  expect(event.kind).toBe(KindSubspaceOp)
  expect(event.subspaceID).toBe('0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef')
  expect(event.operation).toBe(OpPost)
  expect(event.tags).toContainEqual(['d', 'subspace_op']) 
  expect(event.tags).toContainEqual(['sid', '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef']) 
  expect(event.tags).toContainEqual(['ops', OpPost]) 
})

test('SubspaceOpEvent: should set additional properties', () => {
  const event = NewSubspaceOpEvent('0x1234567890abcdef', OpPost, 'Test content')

  SetContentType(event, 'text/plain')
  SetParent(event, '0xabcdef1234567890')
  SetProposal(event, 'proposal123', 'rule1')
  SetVote(event, 'proposal123', 'yes')
  SetInvite(event, '0xinviteepubkey', 'rule2')
  SetContributions(event, '10')

  expect(event.contentType).toBe('text/plain')
  expect(event.parentHash).toBe('0xabcdef1234567890')
  expect(event.proposalID).toBe('proposal123')
  expect(event.vote).toBe('yes')
  expect(event.inviteePubkey).toBe('0xinviteepubkey')
  expect(event.contributions).toBe('10')
  expect(event.tags).toContainEqual(['content_type', 'text/plain']) 
  expect(event.tags).toContainEqual(['parent', '0xabcdef1234567890']) 
  expect(event.tags).toContainEqual(['proposal_id', 'proposal123']) 
  expect(event.tags).toContainEqual(['vote', 'yes']) 
  expect(event.tags).toContainEqual(['invitee_pubkey', '0xinviteepubkey']) 
  expect(event.tags).toContainEqual(['contrib', '10']) 
})

test('SubspaceOpEvent: should validate a valid event', () => {
  const event = NewSubspaceOpEvent('0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef', OpPost,"Test content")
  SetContentType(event, 'text/plain')

  expect(() => ValidateSubspaceOpEvent(event)).not.toThrow()
})

test('SubspaceOpEvent: should throw error for missing required fields', () => {
  const event = NewSubspaceOpEvent('0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef', OpVote, 'Test content')

  expect(() => ValidateSubspaceOpEvent(event)).toThrow('proposal_id and vote are required for vote operation')
})

test('SubspaceOpEvent: should parse a raw event', () => {
  const rawEvent: NostrEvent = {
    kind: KindSubspaceOp,
    content: 'Test content',
    created_at: 1234567890,
    tags: [
      ['d', 'subspace_op'],
      ['sid', '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef'],
      ['ops', OpPost],
      ['content_type', 'text/plain'],
      ['parent', '0xabcdef1234567890'],
      ['proposal_id', 'proposal123'],
    ],
  }
  const parsedEvent = ParseSubspaceOpEvent(rawEvent)

  expect(parsedEvent.kind).toBe(rawEvent.kind)
  expect(parsedEvent.operation).toBe(OpPost)
})
