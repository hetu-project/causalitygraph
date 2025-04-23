package nostr

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSubspaceCreateEvent(t *testing.T) {
	// Test creating a subspace with basic operations
	createEvent := NewSubspaceCreateEvent(
		"test-subspace",
		DefaultSubspaceOps,
		"energy>1000",
		"Test Subspace",
		"https://example.com/image.png",
	)

	// Verify event kind
	assert.Equal(t, KindSubspaceCreate, createEvent.Kind)

	// Verify required tags
	requiredTags := map[string]bool{
		"d":             false,
		"sid":           false,
		"subspace_name": false,
		"ops":           false,
	}

	for _, tag := range createEvent.Tags {
		if len(tag) < 2 {
			continue
		}
		if _, exists := requiredTags[tag[0]]; exists {
			requiredTags[tag[0]] = true
		}
	}

	for tag, found := range requiredTags {
		assert.True(t, found, "missing required tag: %s", tag)
	}

	// Verify sid matches calculated hash
	calculatedSID := calculateSubspaceID(createEvent.SubspaceName, createEvent.Ops, createEvent.Rules)
	assert.Equal(t, calculatedSID, createEvent.SubspaceID)

	// Test validation
	err := ValidateSubspaceCreateEvent(createEvent)
	assert.NoError(t, err)

	// Test parsing
	parsedEvent, err := ParseSubspaceCreateEvent(createEvent.Event)
	assert.NoError(t, err)
	assert.Equal(t, createEvent.SubspaceID, parsedEvent.SubspaceID)
	assert.Equal(t, createEvent.SubspaceName, parsedEvent.SubspaceName)
	assert.Equal(t, createEvent.Ops, parsedEvent.Ops)
	assert.Equal(t, createEvent.Rules, parsedEvent.Rules)
}

func TestSubspaceJoinEvent(t *testing.T) {
	// Create a subspace first to get a valid sid
	createEvent := NewSubspaceCreateEvent(
		"test-subspace",
		DefaultSubspaceOps,
		"energy>1000",
		"Test Subspace",
		"https://example.com/image.png",
	)

	// Test creating a join event
	joinEvent := NewSubspaceJoinEvent(createEvent.SubspaceID)

	// Verify event kind
	assert.Equal(t, KindSubspaceJoin, joinEvent.Kind)

	// Verify required tags
	requiredTags := map[string]bool{
		"d":   false,
		"sid": false,
	}

	for _, tag := range joinEvent.Tags {
		if len(tag) < 2 {
			continue
		}
		if _, exists := requiredTags[tag[0]]; exists {
			requiredTags[tag[0]] = true
		}
	}

	for tag, found := range requiredTags {
		assert.True(t, found, "missing required tag: %s", tag)
	}

	// Test validation
	err := ValidateSubspaceJoinEvent(joinEvent)
	assert.NoError(t, err)

	// Test parsing
	parsedEvent, err := ParseSubspaceJoinEvent(joinEvent.Event)
	assert.NoError(t, err)
	assert.Equal(t, joinEvent.SubspaceID, parsedEvent.SubspaceID)
}

func TestSubspaceOpEvent(t *testing.T) {
	// Create a subspace first to get a valid sid
	createEvent := NewSubspaceCreateEvent(
		"test-subspace",
		DefaultSubspaceOps,
		"energy>1000",
		"Test Subspace",
		"https://example.com/image.png",
	)

	// Test creating a post operation
	postEvent := NewSubspaceOpEvent(createEvent.SubspaceID, OpPost)
	postEvent.SetContentType("markdown")
	postEvent.SetParent("parent-hash")
	postEvent.Content = "Test post content"

	// Verify event kind
	assert.Equal(t, KindSubspaceOp, postEvent.Kind)

	// Verify required tags
	requiredTags := map[string]bool{
		"d":   false,
		"sid": false,
		"ops": false,
	}

	for _, tag := range postEvent.Tags {
		if len(tag) < 2 {
			continue
		}
		if _, exists := requiredTags[tag[0]]; exists {
			requiredTags[tag[0]] = true
		}
	}

	for tag, found := range requiredTags {
		assert.True(t, found, "missing required tag: %s", tag)
	}

	// Test validation
	err := ValidateSubspaceOpEvent(postEvent)
	assert.NoError(t, err)

	// Test parsing
	parsedEvent, err := ParseSubspaceOpEvent(postEvent.Event)
	assert.NoError(t, err)
	assert.Equal(t, postEvent.SubspaceID, parsedEvent.SubspaceID)
	assert.Equal(t, postEvent.Operation, parsedEvent.Operation)
	assert.Equal(t, postEvent.ContentType, parsedEvent.ContentType)
	assert.Equal(t, postEvent.ParentHash, parsedEvent.ParentHash)
}
