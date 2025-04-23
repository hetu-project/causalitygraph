package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"path/filepath"

	ipfsCore "github.com/ipfs/interface-go-ipfs-core"
	"github.com/ipfs/kubo/config"
	"github.com/ipfs/kubo/core"
	"github.com/ipfs/kubo/core/coreapi"
	"github.com/ipfs/kubo/plugin/loader"

	// coreiface "github.com/ipfs/kubo/core/coreiface"
	"github.com/ipfs/kubo/core/node/libp2p"
	// "github.com/ipfs/kubo/plugin/loader"
	"github.com/ipfs/kubo/repo/fsrepo"

	// 导入 IPFS 数据存储驱动
	_ "github.com/ipfs/go-ds-badger"
	_ "github.com/ipfs/go-ds-flatfs"
	_ "github.com/ipfs/go-ds-leveldb"
	_ "github.com/ipfs/go-ds-measure"
)

// InitIPFS 简单初始化 IPFS 节点（只在已存在仓库基础上，不自动初始化新仓库）
func InitIPFS(repoPath string) (ipfsCore.CoreAPI, *core.IpfsNode, error) {
	// 设置默认仓库路径
	if repoPath == "" {
		home, err := os.UserHomeDir()
		if err != nil {
			return nil, nil, fmt.Errorf("获取用户主目录失败: %w", err)
		}
		repoPath = filepath.Join(home, ".ipfs")
	}

	// 如果路径以 ~ 开头，展开到用户主目录
	if len(repoPath) > 0 && repoPath[0] == '~' {
		home, err := os.UserHomeDir()
		if err != nil {
			return nil, nil, fmt.Errorf("获取用户主目录失败: %w", err)
		}
		repoPath = filepath.Join(home, repoPath[1:])
	}

	log.Printf("使用 IPFS 仓库路径: %s", repoPath)
	plugins, err := loader.NewPluginLoader(repoPath)
	if err != nil {
		panic(fmt.Errorf("error loading plugins: %s", err))
	}

	if err := plugins.Initialize(); err != nil {
		panic(fmt.Errorf("error initializing plugins: %s", err))
	}

	if err := plugins.Inject(); err != nil {
		panic(fmt.Errorf("error initializing plugins: %s", err))
	}

	// 检查仓库是否已初始化
	exists := fsrepo.IsInitialized(repoPath)

	// 如果仓库不存在，初始化它
	if !exists {
		log.Printf("初始化 IPFS 仓库: %s", repoPath)
		if err := initRepo(repoPath); err != nil {
			return nil, nil, fmt.Errorf("初始化 IPFS 仓库失败: %w", err)
		}
	}

	// 打开仓库
	repo, err := fsrepo.Open(repoPath)
	if err != nil {
		return nil, nil, fmt.Errorf("打开 IPFS 仓库失败: %w", err)
	}

	// 创建节点
	ctx := context.Background()
	nodeOptions := &core.BuildCfg{
		Online:  true,
		Routing: libp2p.DHTOption,
		Repo:    repo,
	}

	node, err := core.NewNode(ctx, nodeOptions)
	if err != nil {
		repo.Close()
		return nil, nil, fmt.Errorf("创建 IPFS 节点失败: %w", err)
	}

	// 使用 coreapi.NewCoreAPI 获取 API
	api, err := coreapi.NewCoreAPI(node)
	if err != nil {
		node.Close()
		return nil, nil, fmt.Errorf("创建 IPFS API 失败: %w", err)
	}

	log.Println("IPFS 节点初始化成功")
	return api, node, nil
}

// initRepo 初始化 IPFS 仓库
func initRepo(repoPath string) error {
	// 创建目录
	if err := os.MkdirAll(repoPath, 0755); err != nil {
		return err
	}

	// 创建默认配置
	cfg, err := config.Init(os.Stdout, 2048)
	if err != nil {
		return err
	}

	// 配置 IPFS 仓库
	cfg.Addresses.Swarm = []string{
		"/ip4/0.0.0.0/tcp/4001",
		"/ip4/0.0.0.0/udp/4001/quic",
		"/ip6/::/tcp/4001",
		"/ip6/::/udp/4001/quic",
	}
	cfg.Addresses.API = []string{"/ip4/0.0.0.0/tcp/5001"}
	cfg.Addresses.Gateway = []string{"/ip4/0.0.0.0/tcp/8080"}

	// 初始化仓库
	return fsrepo.Init(repoPath, cfg)
}
