package nostr

import (
	"crypto/sha256"
	"encoding/hex"
	jsonutils "encoding/json"
	"fmt"
	"strings"
	"time"
)

// Subspace event kinds
const (
	KindSubspaceCreate = 30100
	KindSubspaceJoin   = 30200
	KindSubspaceOp     = 30300
)

// Basic operation types (core operations)
const (
	OpPost    = "post"    // 1
	OpPropose = "propose" // 2
	OpVote    = "vote"    // 3
	OpInvite  = "invite"  // 4
)

// Default operations string for subspace creation
const DefaultSubspaceOps = "post=1,propose=2,vote=3,invite=4"

// SubspaceCreateEvent represents a subspace creation event
type SubspaceCreateEvent struct {
	Event
	SubspaceID   string
	SubspaceName string
	Ops          string
	Rules        string
	Description  string
	ImageURL     string
}

// calculateSubspaceID generates a unique subspace ID based on subspace_name, ops, and rules
func calculateSubspaceID(subspaceName, ops, rules string) string {
	// Concatenate the components
	input := subspaceName + ops + rules
	// Calculate SHA256 hash
	hash := sha256.Sum256([]byte(input))
	// Convert to hex string with "0x" prefix
	return "0x" + hex.EncodeToString(hash[:])
}

// NewSubspaceCreateEvent creates a new subspace creation event
func NewSubspaceCreateEvent(subspaceName, ops, rules, description, imageURL string) *SubspaceCreateEvent {
	// Calculate subspace ID
	sid := calculateSubspaceID(subspaceName, ops, rules)

	evt := &SubspaceCreateEvent{
		Event: Event{
			Kind:      KindSubspaceCreate,
			CreatedAt: Timestamp(time.Now().Unix()),
		},
		SubspaceID:   sid,
		SubspaceName: subspaceName,
		Ops:          ops,
		Rules:        rules,
		Description:  description,
		ImageURL:     imageURL,
	}

	// Set tags
	evt.Tags = Tags{
		Tag{"d", "subspace_create"},
		Tag{"sid", sid},
		Tag{"subspace_name", subspaceName},
		Tag{"ops", ops},
	}
	if rules != "" {
		evt.Tags = append(evt.Tags, Tag{"rules", rules})
	}

	// Set content
	content := map[string]string{
		"desc":    description,
		"img_url": imageURL,
	}
	contentBytes, _ := jsonutils.Marshal(content)
	evt.Content = string(contentBytes)

	return evt
}

// ValidateSubspaceCreateEvent validates a SubspaceCreateEvent
func ValidateSubspaceCreateEvent(evt *SubspaceCreateEvent) error {
	// 1. Verify event kind
	if evt.Kind != KindSubspaceCreate {
		return fmt.Errorf("invalid event kind: expected %d, got %d", KindSubspaceCreate, evt.Kind)
	}

	// 2. Verify required tags
	requiredTags := map[string]bool{
		"d":             false,
		"sid":           false,
		"subspace_name": false,
		"ops":           false,
	}

	for _, tag := range evt.Tags {
		if len(tag) < 2 {
			continue
		}
		if _, exists := requiredTags[tag[0]]; exists {
			requiredTags[tag[0]] = true
		}
	}

	// Check if all required tags are present
	for tag, found := range requiredTags {
		if !found {
			return fmt.Errorf("missing required tag: %s", tag)
		}
	}

	// 3. Verify sid matches the calculated hash
	calculatedSID := calculateSubspaceID(evt.SubspaceName, evt.Ops, evt.Rules)
	if evt.SubspaceID != calculatedSID {
		return fmt.Errorf("invalid subspace ID: expected %s, got %s", calculatedSID, evt.SubspaceID)
	}

	// 4. Verify content is valid JSON with required fields
	var content struct {
		Desc   string `json:"desc"`
		ImgURL string `json:"img_url"`
	}
	if err := jsonutils.Unmarshal([]byte(evt.Content), &content); err != nil {
		return fmt.Errorf("invalid content format: %v", err)
	}
	if content.Desc == "" {
		return fmt.Errorf("missing description in content")
	}

	// 5. Verify ops format
	// ops should be in format "key1=value1,key2=value2,..."
	opsParts := strings.Split(evt.Ops, ",")
	for _, part := range opsParts {
		if !strings.Contains(part, "=") {
			return fmt.Errorf("invalid ops format: %s", evt.Ops)
		}
	}

	return nil
}

// ParseSubspaceCreateEvent parses a raw Event into a SubspaceCreateEvent
func ParseSubspaceCreateEvent(evt Event) (*SubspaceCreateEvent, error) {
	// Create new SubspaceCreateEvent
	subspaceEvt := &SubspaceCreateEvent{
		Event: evt,
	}

	// Extract fields from tags
	for _, tag := range evt.Tags {
		if len(tag) < 2 {
			continue
		}
		switch tag[0] {
		case "sid":
			subspaceEvt.SubspaceID = tag[1]
		case "subspace_name":
			subspaceEvt.SubspaceName = tag[1]
		case "ops":
			subspaceEvt.Ops = tag[1]
		case "rules":
			subspaceEvt.Rules = tag[1]
		}
	}

	// Parse content
	var content struct {
		Desc   string `json:"desc"`
		ImgURL string `json:"img_url"`
	}
	if err := jsonutils.Unmarshal([]byte(evt.Content), &content); err != nil {
		return nil, fmt.Errorf("failed to parse content: %v", err)
	}
	subspaceEvt.Description = content.Desc
	subspaceEvt.ImageURL = content.ImgURL

	// Validate the parsed event
	if err := ValidateSubspaceCreateEvent(subspaceEvt); err != nil {
		return nil, fmt.Errorf("invalid subspace create event: %v", err)
	}

	return subspaceEvt, nil
}

// SubspaceJoinEvent represents a subspace join event
type SubspaceJoinEvent struct {
	Event
	SubspaceID string
}

// NewSubspaceJoinEvent creates a new subspace join event
func NewSubspaceJoinEvent(subspaceID string) *SubspaceJoinEvent {
	evt := &SubspaceJoinEvent{
		Event: Event{
			Kind:      KindSubspaceJoin,
			CreatedAt: Timestamp(time.Now().Unix()),
		},
		SubspaceID: subspaceID,
	}

	evt.Tags = Tags{
		Tag{"d", "subspace_join"},
		Tag{"sid", subspaceID},
	}

	return evt
}

// ValidateSubspaceJoinEvent validates a SubspaceJoinEvent
func ValidateSubspaceJoinEvent(evt *SubspaceJoinEvent) error {
	// 1. Verify event kind
	if evt.Kind != KindSubspaceJoin {
		return fmt.Errorf("invalid event kind: expected %d, got %d", KindSubspaceJoin, evt.Kind)
	}

	// 2. Verify required tags
	requiredTags := map[string]bool{
		"d":   false,
		"sid": false,
	}

	for _, tag := range evt.Tags {
		if len(tag) < 2 {
			continue
		}
		if _, exists := requiredTags[tag[0]]; exists {
			requiredTags[tag[0]] = true
		}
	}

	// Check if all required tags are present
	for tag, found := range requiredTags {
		if !found {
			return fmt.Errorf("missing required tag: %s", tag)
		}
	}

	// 3. Verify sid format (should be a valid hex string with 0x prefix)
	if !strings.HasPrefix(evt.SubspaceID, "0x") {
		return fmt.Errorf("invalid subspace ID format: should start with 0x")
	}
	if len(evt.SubspaceID) != 66 { // 0x + 64 hex chars
		return fmt.Errorf("invalid subspace ID length: expected 66, got %d", len(evt.SubspaceID))
	}

	return nil
}

// ParseSubspaceJoinEvent parses a raw Event into a SubspaceJoinEvent
func ParseSubspaceJoinEvent(evt Event) (*SubspaceJoinEvent, error) {
	joinEvt := &SubspaceJoinEvent{
		Event: evt,
	}

	// Extract fields from tags
	for _, tag := range evt.Tags {
		if len(tag) < 2 {
			continue
		}
		if tag[0] == "sid" {
			joinEvt.SubspaceID = tag[1]
		}
	}

	// Validate the parsed event
	if err := ValidateSubspaceJoinEvent(joinEvt); err != nil {
		return nil, fmt.Errorf("invalid subspace join event: %v", err)
	}

	return joinEvt, nil
}

// SubspaceOpEvent represents a subspace operation event
type SubspaceOpEvent struct {
	Event
	SubspaceID    string
	Operation     string
	ContentType   string
	ParentHash    string
	ProposalID    string
	Vote          string
	InviteePubkey string
	Contributions string
}

// NewSubspaceOpEvent creates a new subspace operation event
func NewSubspaceOpEvent(subspaceID, operation string) *SubspaceOpEvent {
	evt := &SubspaceOpEvent{
		Event: Event{
			Kind:      KindSubspaceOp,
			CreatedAt: Timestamp(time.Now().Unix()),
		},
		SubspaceID: subspaceID,
		Operation:  operation,
	}

	evt.Tags = Tags{
		Tag{"d", "subspace_op"},
		Tag{"sid", subspaceID},
		Tag{"ops", operation},
	}

	return evt
}

// SetContentType sets the content type for the operation
func (e *SubspaceOpEvent) SetContentType(contentType string) {
	e.ContentType = contentType
	e.Tags = append(e.Tags, Tag{"content_type", contentType})
}

// SetParent sets the parent event hash
func (e *SubspaceOpEvent) SetParent(parentHash string) {
	e.ParentHash = parentHash
	e.Tags = append(e.Tags, Tag{"parent", parentHash})
}

// SetProposal sets the proposal ID and rules
func (e *SubspaceOpEvent) SetProposal(proposalID, rules string) {
	e.ProposalID = proposalID
	e.Tags = append(e.Tags, Tag{"proposal_id", proposalID})
	if rules != "" {
		e.Tags = append(e.Tags, Tag{"rules", rules})
	}
}

// SetVote sets the vote for a proposal
func (e *SubspaceOpEvent) SetVote(proposalID, vote string) {
	e.ProposalID = proposalID
	e.Vote = vote
	e.Tags = append(e.Tags, Tag{"proposal_id", proposalID}, Tag{"vote", vote})
}

// SetInvite sets the invitee pubkey and rules
func (e *SubspaceOpEvent) SetInvite(inviteePubkey, rules string) {
	e.InviteePubkey = inviteePubkey
	e.Tags = append(e.Tags, Tag{"invitee_pubkey", inviteePubkey})
	if rules != "" {
		e.Tags = append(e.Tags, Tag{"rules", rules})
	}
}

// SetContributions sets the contribution weights
func (e *SubspaceOpEvent) SetContributions(contributions string) {
	e.Contributions = contributions
	e.Tags = append(e.Tags, Tag{"contrib", contributions})
}

// ValidateSubspaceOpEvent validates a SubspaceOpEvent
func ValidateSubspaceOpEvent(evt *SubspaceOpEvent) error {
	// 1. Verify event kind
	if evt.Kind != KindSubspaceOp {
		return fmt.Errorf("invalid event kind: expected %d, got %d", KindSubspaceOp, evt.Kind)
	}

	// 2. Verify required tags
	requiredTags := map[string]bool{
		"d":   false,
		"sid": false,
		"ops": false,
	}

	for _, tag := range evt.Tags {
		if len(tag) < 2 {
			continue
		}
		if _, exists := requiredTags[tag[0]]; exists {
			requiredTags[tag[0]] = true
		}
	}

	// Check if all required tags are present
	for tag, found := range requiredTags {
		if !found {
			return fmt.Errorf("missing required tag: %s", tag)
		}
	}

	// 3. Verify sid format
	if !strings.HasPrefix(evt.SubspaceID, "0x") {
		return fmt.Errorf("invalid subspace ID format: should start with 0x")
	}
	if len(evt.SubspaceID) != 66 {
		return fmt.Errorf("invalid subspace ID length: expected 66, got %d", len(evt.SubspaceID))
	}

	// 4. Operation-specific validations
	switch evt.Operation {
	case OpPost:
		if evt.ContentType == "" {
			return fmt.Errorf("content_type is required for post operation")
		}
	case OpPropose:
		if evt.ProposalID == "" {
			return fmt.Errorf("proposal_id is required for propose operation")
		}
	case OpVote:
		if evt.ProposalID == "" || evt.Vote == "" {
			return fmt.Errorf("proposal_id and vote are required for vote operation")
		}
		if evt.Vote != "yes" && evt.Vote != "no" {
			return fmt.Errorf("invalid vote value: %s", evt.Vote)
		}
	case OpInvite:
		if evt.InviteePubkey == "" {
			return fmt.Errorf("invitee_pubkey is required for invite operation")
		}
	}

	return nil
}

// ParseSubspaceOpEvent parses a raw Event into a SubspaceOpEvent
func ParseSubspaceOpEvent(evt Event) (*SubspaceOpEvent, error) {
	opEvt := &SubspaceOpEvent{
		Event: evt,
	}

	// Extract fields from tags
	for _, tag := range evt.Tags {
		if len(tag) < 2 {
			continue
		}
		switch tag[0] {
		case "sid":
			opEvt.SubspaceID = tag[1]
		case "ops":
			opEvt.Operation = tag[1]
		case "content_type":
			opEvt.ContentType = tag[1]
		case "parent":
			opEvt.ParentHash = tag[1]
		case "proposal_id":
			opEvt.ProposalID = tag[1]
		case "vote":
			opEvt.Vote = tag[1]
		case "invitee_pubkey":
			opEvt.InviteePubkey = tag[1]
		case "contrib":
			opEvt.Contributions = tag[1]
		}
	}

	// Validate the parsed event
	if err := ValidateSubspaceOpEvent(opEvt); err != nil {
		return nil, fmt.Errorf("invalid subspace operation event: %v", err)
	}

	return opEvt, nil
}
