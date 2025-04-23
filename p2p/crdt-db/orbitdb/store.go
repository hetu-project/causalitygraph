package orbitdb

import (
	"context"
	"encoding/json"
	"fmt"
	"log"

	"berty.tech/go-orbit-db/iface"
	"github.com/nbd-wtf/go-nostr"
)

// OrbitDBAdapter 实现NostrStore接口
type OrbitDBAdapter struct {
	db iface.DocumentStore
}

// NewOrbitDBAdapter 创建一个新的OrbitDB适配器
func NewOrbitDBAdapter(db iface.DocumentStore) *OrbitDBAdapter {
	return &OrbitDBAdapter{
		db: db,
	}
}

// SaveEvent 保存nostr事件
func (a *OrbitDBAdapter) SaveEvent(event *nostr.Event) error {
	if event == nil {
		return fmt.Errorf("event cannot be nil")
	}

	// 将事件转换为map
	eventJSON, err := json.Marshal(event)
	if err != nil {
		return fmt.Errorf("failed to marshal event: %w", err)
	}

	var eventMap map[string]interface{}
	if err = json.Unmarshal(eventJSON, &eventMap); err != nil {
		return fmt.Errorf("failed to unmarshal event: %w", err)
	}

	// 使用事件ID作为文档ID
	eventMap["_id"] = event.ID

	// 保存到OrbitDB
	_, err = a.db.Put(context.Background(), eventMap)
	if err != nil {
		return fmt.Errorf("failed to save event: %w", err)
	}

	log.Printf("Event saved: %s", event.ID)
	return nil
}

// GetEvent 通过ID获取事件
func (a *OrbitDBAdapter) GetEvent(id string) (*nostr.Event, error) {
	docs, err := a.db.Get(context.Background(), id, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to get event: %w", err)
	}

	if len(docs) == 0 {
		return nil, fmt.Errorf("event not found: %s", id)
	}

	// 转换回nostr.Event
	eventJSON, err := json.Marshal(docs[0])
	if err != nil {
		return nil, fmt.Errorf("failed to marshal doc: %w", err)
	}

	var event nostr.Event
	if err = json.Unmarshal(eventJSON, &event); err != nil {
		return nil, fmt.Errorf("failed to unmarshal event: %w", err)
	}

	return &event, nil
}

// QueryEvents 根据过滤器查询事件
func (a *OrbitDBAdapter) QueryEvents(filter nostr.Filter) ([]*nostr.Event, error) {
	var results []*nostr.Event

	// 定义查询函数
	queryFn := func(doc interface{}) (bool, error) {
		event, ok := doc.(map[string]interface{})
		if !ok {
			return false, nil
		}

		// 实现过滤逻辑
		if len(filter.IDs) > 0 {
			id, ok := event["id"].(string)
			if !ok || !contains(filter.IDs, id) {
				return false, nil
			}
		}

		if len(filter.Authors) > 0 {
			pubkey, ok := event["pubkey"].(string)
			if !ok || !contains(filter.Authors, pubkey) {
				return false, nil
			}
		}

		if len(filter.Kinds) > 0 {
			kind, ok := event["kind"].(float64)
			if !ok || !containsInt(filter.Kinds, int(kind)) {
				return false, nil
			}
		}

		return true, nil
	}

	// 执行查询
	docs, _ := a.db.Query(context.Background(), queryFn)
	for _, doc := range docs {
		eventJSON, err := json.Marshal(doc)
		if err != nil {
			continue
		}

		var event nostr.Event
		if err = json.Unmarshal(eventJSON, &event); err != nil {
			continue
		}

		results = append(results, &event)
	}

	return results, nil
}

// DeleteEvent 删除事件
func (a *OrbitDBAdapter) DeleteEvent(id string) error {
	docs, err := a.db.Get(context.Background(), id, nil)
	if err != nil {
		return fmt.Errorf("failed to get event: %w", err)
	}

	if len(docs) == 0 {
		return fmt.Errorf("event not found: %s", id)
	}

	_, err = a.db.Delete(context.Background(), id)
	if err != nil {
		return fmt.Errorf("failed to delete event: %w", err)
	}

	return nil
}

// 辅助函数
func contains(slice []string, item string) bool {
	for _, s := range slice {
		if s == item {
			return true
		}
	}
	return false
}

func containsInt(slice []int, item int) bool {
	for _, s := range slice {
		if s == item {
			return true
		}
	}
	return false
}
