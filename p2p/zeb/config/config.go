package config

import "github.com/hetu-project/zeb/dht"

const DefaultRpcPort = 13333
const DefaultP2pPort = 33333
const DefaultWsPort = 23333
const DefaultUdpPort = 8050
const DefaultVlcAddr = "127.0.0.1:8050"

type Config struct {
	Transport string
	P2pPort   uint16
	Keypair   dht.KeyPair
	WsPort    uint16
	RpcPort   uint16
	UdpPort   uint16
	VlcAddr   string
	Domain    string
	SeedList  []string
}
