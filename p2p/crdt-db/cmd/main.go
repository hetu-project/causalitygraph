package main

import (
	"context"
	// "encoding/json"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"
	"strings"

	orbitdb "berty.tech/go-orbit-db"
	"berty.tech/go-orbit-db/accesscontroller"
	"berty.tech/go-orbit-db/iface"
	shell "github.com/ipfs/go-ipfs-api"
	core "github.com/ipfs/kubo/core"
	"github.com/ipfs/kubo/core/coreapi"

	// coreapi "github.com/ipfs/kubo/client/rpc"
	"github.com/libp2p/go-libp2p"
	"github.com/libp2p/go-libp2p/core/crypto"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	ma "github.com/multiformats/go-multiaddr"

	// Import IPFS data storage drivers
	_ "github.com/ipfs/go-ds-badger"
	_ "github.com/ipfs/go-ds-flatfs"
	_ "github.com/ipfs/go-ds-leveldb"
	_ "github.com/ipfs/go-ds-measure"
	// "github.com/ipfs/kubo/core/node/libp2p"
)

var (
	dbAddress  = flag.String("db", "", "OrbitDB address to connect to")
	dataDir    = flag.String("data", "~/data", "Data directory path")
	listenAddr = flag.String("listen", "/ip4/0.0.0.0/tcp/4001", "Libp2p listen address")
	ipfssAPI   = flag.String("ipfs", "localhost:5001", "IPFS API endpoint")
	Create     = true
)

func main() {
	flag.Parse()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Setup data directories
	ipfsDir := filepath.Join(*dataDir, "ipfs")
	orbitDBDir := filepath.Join(*dataDir, "orbitdb")
	settingsDir := filepath.Join(*dataDir, "settings")

	// Ensure directories exist
	for _, dir := range []string{ipfsDir, orbitDBDir, settingsDir} {
		if err := os.MkdirAll(dir, 0755); err != nil {
			log.Fatalf("Failed to create directory %s: %v", dir, err)
		}
	}

	// Get or generate peer identity
	privKey, peerID, err := getOrCreatePeerID(settingsDir)
	if err != nil {
		log.Fatalf("Failed to get peer ID: %v", err)
	}
	log.Printf("Using Peer ID: %s", peerID.String())

	// Connect to local IPFS node
	// ipfsAPI, ipfsNode, err := InitIPFS(ipfsDir)
	// if err != nil {
	// 	log.Fatalf("Failed to initialize IPFS: %v", err)
	// }
	// defer ipfsNode.Close()
	node, _ := core.NewNode(ctx, &core.BuildCfg{
		Online:  true,  // 必须为 true，OrbitDB 需要网络功能
		NilRepo: false, // 需要持久化存储
		ExtraOpts: map[string]bool{
			"pubsub": true, // OrbitDB 依赖 PubSub
			"mplex":  true, // 多路复用支持
		},
	})
	api, _ := coreapi.NewCoreAPI(node)
	// Initialize IPFS HTTP client
	sh := shell.NewShell(*ipfssAPI)
	if sh == nil {
		log.Fatalf("Failed to initialize IPFS HTTP client")
	}
	// 2. 转换为 coreapi 接口
	// api, err := coreapi.NewClient(sh)
	// if err != nil {
	// 	log.Fatal(err)
	// }
	// Setup libp2p host
	host, err := setupLibp2p(ctx, privKey, *listenAddr)
	if err != nil {
		log.Fatalf("Failed to create libp2p host: %v", err)
	}

	// Print peer addresses
	addrs := host.Addrs()
	var addrStrings []string
	for _, addr := range addrs {
		addrStrings = append(addrStrings, fmt.Sprintf("%s/p2p/%s", addr.String(), host.ID().String()))
	}
	log.Printf("Peer addresses: %s", strings.Join(addrStrings, ", "))

	// Create OrbitDB instance
	// orbit, err := orbitdb.NewOrbitDB(ctx, ipfsAPI, &orbitdb.NewOrbitDBOptions{
	// 	Directory: &orbitDBDir,
	// })
	// Explicitly try to enable pubsub
	// Create OrbitDB instance with explicit pubsub options
	orbit, err := orbitdb.NewOrbitDB(ctx, api, &orbitdb.NewOrbitDBOptions{
		Directory: &orbitDBDir,
	})
	if err != nil {
		log.Fatalf("Failed to create OrbitDB instance: %v", err)
	}
	// Open or create database
	var db iface.DocumentStore
	if *dbAddress != "" {
		// Connect to existing database
		log.Printf("Connecting to database: %s", *dbAddress)
		dbInstance, err := orbit.Open(ctx, *dbAddress, &orbitdb.CreateDBOptions{
			Directory: &orbitDBDir,
			Create:    &Create,
		})
		if err != nil {
			log.Fatalf("Failed to open database: %v", err)
		}
		db = dbInstance.(iface.DocumentStore)
	} else {
		// Create new database with open write access
		log.Printf("Creating new database")
		dbName := "nostr-events"

		// Create database options, including index settings
		// storeOptions := map[string]interface{}{
		// 	"indexBy": "_id", // Use _id as the primary index
		// }

		// indexBy := "_id"
		dbOptions := &orbitdb.CreateDBOptions{
			AccessController: &accesscontroller.CreateAccessControllerOptions{
				Type: "ipfs",
				Access: map[string][]string{
					"write": {"*"}, // Allow anyone to write
				},
			},
			Directory: &orbitDBDir,
			Create:    &Create,
			// StoreSpecificOpts: storeOptions,
		}

		dbInstance, err := orbit.Docs(ctx, dbName, dbOptions)
		if err != nil {
			log.Fatalf("Failed to create database: %v", err)
		}
		db = dbInstance
		log.Printf("Database created with address: %s", db.Address().String())
	}
	defer db.Close()

}

// getOrCreatePeerID loads or creates a peer ID
func getOrCreatePeerID(settingsDir string) (crypto.PrivKey, peer.ID, error) {
	keyFile := filepath.Join(settingsDir, "peer.key")

	// Check if key file exists
	if _, err := os.Stat(keyFile); os.IsNotExist(err) {
		// Generate new key
		priv, pub, err := crypto.GenerateKeyPair(crypto.Ed25519, -1)
		if err != nil {
			return nil, "", fmt.Errorf("failed to generate key pair: %w", err)
		}

		// Get peer ID from public key
		pid, err := peer.IDFromPublicKey(pub)
		if err != nil {
			return nil, "", fmt.Errorf("failed to get peer ID: %w", err)
		}

		// Serialize private key
		keyBytes, err := crypto.MarshalPrivateKey(priv)
		if err != nil {
			return nil, "", fmt.Errorf("failed to marshal private key: %w", err)
		}

		// Save to file
		if err := ioutil.WriteFile(keyFile, keyBytes, 0600); err != nil {
			return nil, "", fmt.Errorf("failed to save key: %w", err)
		}

		log.Printf("Generated new peer ID: %s", pid.String())
		return priv, pid, nil
	}

	// Load existing key
	keyBytes, err := ioutil.ReadFile(keyFile)
	if err != nil {
		return nil, "", fmt.Errorf("failed to read key file: %w", err)
	}

	// Unmarshal private key
	priv, err := crypto.UnmarshalPrivateKey(keyBytes)
	if err != nil {
		return nil, "", fmt.Errorf("failed to unmarshal private key: %w", err)
	}

	// Get peer ID from public key
	pub := priv.GetPublic()
	pid, err := peer.IDFromPublicKey(pub)
	if err != nil {
		return nil, "", fmt.Errorf("failed to get peer ID: %w", err)
	}

	log.Printf("Loaded existing peer ID: %s", pid.String())
	return priv, pid, nil
}

// setupLibp2p creates a libp2p host
func setupLibp2p(ctx context.Context, privKey crypto.PrivKey, listenAddr string) (host.Host, error) {
	addr, err := ma.NewMultiaddr(listenAddr)
	if err != nil {
		return nil, fmt.Errorf("invalid listen address: %w", err)
	}

	// Create libp2p host with only necessary options
	host, err := libp2p.New(
		libp2p.ListenAddrs(addr),
		libp2p.Identity(privKey),
		libp2p.EnableRelay(),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create libp2p host: %w", err)
	}

	return host, nil
}
