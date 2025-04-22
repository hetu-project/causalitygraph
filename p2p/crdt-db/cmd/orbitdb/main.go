package main

import (
	"context"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"os/signal"
	"path/filepath"
	"strings"
	"syscall"
	"time"

	orbitdb "berty.tech/go-orbit-db"
	"berty.tech/go-orbit-db/accesscontroller"
	"berty.tech/go-orbit-db/iface"
	"berty.tech/go-orbit-db/stores/kvstore"
	shell "github.com/ipfs/go-ipfs-api"
	"github.com/libp2p/go-libp2p"
	"github.com/libp2p/go-libp2p/core/crypto"
	"github.com/libp2p/go-libp2p/core/peer"
	ma "github.com/multiformats/go-multiaddr"
)

var (
	dbAddress  = flag.String("db", "", "OrbitDB address to connect to")
	dataDir    = flag.String("data", "./data", "Data directory path")
	listenAddr = flag.String("listen", "/ip4/0.0.0.0/tcp/4001", "Libp2p listen address")
	ipfsAPI    = flag.String("ipfs", "localhost:5001", "IPFS API endpoint")
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
	sh := shell.NewShell(*ipfsAPI)
	if _, err := sh.ID(); err != nil {
		log.Fatalf("Failed to connect to IPFS: %v", err)
	}

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
	orbit, err := orbitdb.NewOrbitDB(ctx, sh, &orbitdb.NewOrbitDBOptions{
		Directory: orbitDBDir,
	})
	if err != nil {
		log.Fatalf("Failed to create OrbitDB instance: %v", err)
	}
	defer orbit.Close()

	// Open or create database
	var db iface.KeyValueStore
	if *dbAddress != "" {
		// Connect to existing database
		log.Printf("Connecting to database: %s", *dbAddress)
		dbInstance, err := orbit.Open(ctx, *dbAddress, &orbitdb.CreateDBOptions{
			Directory: orbitDBDir,
			Create:    true,
		})
		if err != nil {
			log.Fatalf("Failed to open database: %v", err)
		}
		db = dbInstance.(iface.KeyValueStore)
	} else {
		// Create new database with open write access
		log.Printf("Creating new database")
		acOptions := &accesscontroller.CreateAccessControllerOptions{
			Access: map[string][]string{
				"write": {"*"}, // Allow anyone to write
			},
		}

		dbInstance, err := orbit.KeyValue(ctx, "onmydisk", &orbitdb.CreateDBOptions{
			AccessController: accesscontroller.NewIPFSAccessController(ctx, acOptions),
			Directory:        orbitDBDir,
			Create:           true,
		})
		if err != nil {
			log.Fatalf("Failed to create database: %v", err)
		}
		db = dbInstance
		log.Printf("Database created with address: %s", db.Address().String())
	}
	defer db.Close()

	// Setup DB event listener
	go func() {
		events := db.EventBus().Subscribe(new(kvstore.EventWrite))
		defer db.EventBus().Unsubscribe(events)

		for evt := range events {
			event, ok := evt.(*kvstore.EventWrite)
			if !ok {
				continue
			}
			log.Printf("DB updated: Key=%s", event.Key)

			// Get the value
			val, err := db.Get(ctx, event.Key)
			if err != nil {
				log.Printf("Error getting value: %v", err)
				continue
			}

			// Parse the value to extract CID
			valueMap, ok := val.(map[string]interface{})
			if !ok {
				log.Printf("Invalid value format: %v", val)
				continue
			}

			cid, ok := valueMap["cid"].(string)
			if !ok {
				log.Printf("No CID in value: %v", valueMap)
				continue
			}

			// Fetch file from IPFS
			log.Printf("Fetching file with CID: %s", cid)
			reader, err := sh.Cat(cid)
			if err != nil {
				log.Printf("Failed to fetch file: %v", err)
				continue
			}

			content, err := ioutil.ReadAll(reader)
			if err != nil {
				log.Printf("Failed to read file: %v", err)
				continue
			}

			log.Printf("Fetched file contents: %s", string(content))
		}
	}()

	// Add a random text file to IPFS
	randomText := fmt.Sprintf("Text Message: %d", time.Now().UnixNano())
	log.Printf("Creating random text: %s", randomText)

	cid, err := sh.Add(strings.NewReader(randomText))
	if err != nil {
		log.Fatalf("Failed to add text to IPFS: %v", err)
	}
	log.Printf("Added file to IPFS: %s", cid)

	// Store in OrbitDB
	value := map[string]interface{}{
		"peer": host.ID().String(),
		"text": randomText,
		"cid":  cid,
	}

	if err := db.Put(ctx, host.ID().String(), value); err != nil {
		log.Fatalf("Failed to put value in OrbitDB: %v", err)
	}
	log.Printf("Added entry to OrbitDB with key: %s", host.ID().String())

	// Keep running until interrupted
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	<-sigChan

	log.Println("Shutting down...")
	// Print final state of DB
	allEntries, err := db.All(ctx)
	if err != nil {
		log.Printf("Failed to get all entries: %v", err)
	} else {
		log.Printf("Final DB state: %+v", allEntries)
	}
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
func setupLibp2p(ctx context.Context, privKey crypto.PrivKey, listenAddr string) (peer.PeerHost, error) {
	addr, err := ma.NewMultiaddr(listenAddr)
	if err != nil {
		return nil, fmt.Errorf("invalid listen address: %w", err)
	}

	// Create libp2p host
	host, err := libp2p.New(
		libp2p.ListenAddrs(addr),
		libp2p.Identity(privKey),
		libp2p.EnableRelay(),
		libp2p.EnableAutoRelay(),
		libp2p.EnableNATService(),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create libp2p host: %w", err)
	}

	return host, nil
}
