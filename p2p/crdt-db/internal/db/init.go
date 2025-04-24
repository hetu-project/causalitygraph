package db

import (
	"context"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"sync"

	orbitdb "berty.tech/go-orbit-db"
	"berty.tech/go-orbit-db/accesscontroller"
	"berty.tech/go-orbit-db/iface"
	ipfsCore "github.com/ipfs/kubo/core"
	"github.com/ipfs/kubo/core/coreapi"
)

var (
	ipfsNode    *ipfsCore.IpfsNode
	orbitDB     iface.OrbitDB
	documentDB  iface.DocumentStore
	initOnce    sync.Once
	initialized bool
)

// Init 初始化数据库连接
func Init() error {
	var initErr error

	initOnce.Do(func() {
		// 获取数据目录
		home, err := os.UserHomeDir()
		if err != nil {
			initErr = fmt.Errorf("无法获取用户主目录: %w", err)
			return
		}

		dataDir := filepath.Join(home, "data")
		ipfsDir := filepath.Join(dataDir, "ipfs")
		orbitDBDir := filepath.Join(dataDir, "orbitdb")

		// 创建必要的目录
		for _, dir := range []string{dataDir, ipfsDir, orbitDBDir} {
			if err := os.MkdirAll(dir, 0755); err != nil {
				initErr = fmt.Errorf("创建目录失败 %s: %w", dir, err)
				return
			}
		}

		// 初始化 IPFS 节点
		ctx := context.Background()
		ipfsNode, err = ipfsCore.NewNode(ctx, &ipfsCore.BuildCfg{
			Online:  true,
			NilRepo: false,
			ExtraOpts: map[string]bool{
				"pubsub": true,
				"mplex":  true,
			},
		})
		if err != nil {
			initErr = fmt.Errorf("初始化 IPFS 节点失败: %w", err)
			return
		}

		// 获取 IPFS API
		api, err := coreapi.NewCoreAPI(ipfsNode)
		if err != nil {
			initErr = fmt.Errorf("创建 IPFS API 失败: %w", err)
			return
		}

		// 创建 OrbitDB 实例
		orbitDB, err = orbitdb.NewOrbitDB(ctx, api, &orbitdb.NewOrbitDBOptions{
			Directory: &orbitDBDir,
		})
		if err != nil {
			initErr = fmt.Errorf("创建 OrbitDB 实例失败: %w", err)
			return
		}

		// 创建文档数据库
		create := true
		dbName := "nostr-events"
		dbOptions := &orbitdb.CreateDBOptions{
			AccessController: &accesscontroller.CreateAccessControllerOptions{
				Type: "ipfs",
				Access: map[string][]string{
					"write": {"*"},
				},
			},
			Directory: &orbitDBDir,
			Create:    &create,
		}

		db, err := orbitDB.Docs(ctx, dbName, dbOptions)
		if err != nil {
			initErr = fmt.Errorf("创建文档数据库失败: %w", err)
			return
		}
		documentDB = db

		initialized = true
		log.Println("数据库初始化成功")
	})

	return initErr
}

// GetStore 获取已初始化的 OrbitDB 存储实例
func GetStore() (iface.DocumentStore, error) {
	if !initialized || documentDB == nil {
		return nil, fmt.Errorf("数据库未初始化")
	}
	return documentDB, nil
}

// Close 关闭数据库连接
func Close() error {
	if documentDB != nil {
		documentDB.Close()
	}

	if orbitDB != nil {
		orbitDB.Close()
	}

	if ipfsNode != nil {
		if err := ipfsNode.Close(); err != nil {
			return fmt.Errorf("关闭 IPFS 节点失败: %w", err)
		}
	}

	initialized = false
	return nil
}
