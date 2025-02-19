package examples

import (
	"fmt"
	"github.com/hetu-project/zeb/config"
	"github.com/hetu-project/zeb/dht"
	"github.com/hetu-project/zeb/utils"
	"github.com/hetu-project/zeb/znode"
	"golang.org/x/crypto/sha3"
	"log"
	"time"
)

const seed = "http://127.0.0.1:12345/rpc12345"

func StartSeed() *znode.Znode {
	h := sha3.New256().Sum([]byte("Hello" + string(rune(100))))
	keypair, _ := dht.GenerateKeyPair(h[:32])
	c := config.Config{
		Transport: "tcp",
		P2pPort:   uint16(12344),
		Keypair:   keypair,
		WsPort:    uint16(12346),
		RpcPort:   uint16(12345),
		UdpPort:   config.DefaultUdpPort,
		VlcAddr:   "127.0.0.1:8050",
		Domain:    "127.0.0.1",
	}

	znd, err := znode.NewZnode(c)
	if err != nil {
		log.Fatal(err)
	}

	znd.ApplyBytesReceived()
	znd.ApplyNeighborAdded()
	znd.ApplyNeighborRemoved()
	znd.ApplyVlcOnRelay()

	znd.Start(true)

	return znd
}

func StartCluster(s *znode.Znode) {
	p2pPort := 33333
	wsPort := 23333
	rpcPort := 13333
	sl := make([]string, 0)
	sl = append(sl, seed)
	zebs := make([]*znode.Znode, 0)

	vlcports := []string{"8010", "8020", "8030", "8040", "8051", "8060", "8070", "8080", "8090", "8100"}

	for i := 0; i < 10; i++ {
		h := sha3.New256().Sum([]byte("Hello" + string(rune(i))))
		keypair, _ := dht.GenerateKeyPair(h[:32])

		p2p := p2pPort + i
		ws := wsPort + i
		rpc := rpcPort + i

		c := config.Config{
			Transport: "tcp",
			P2pPort:   uint16(p2p),
			Keypair:   keypair,
			WsPort:    uint16(ws),
			RpcPort:   uint16(rpc),
			UdpPort:   config.DefaultUdpPort,
			VlcAddr:   "127.0.0.1:" + vlcports[i],
			SeedList:  sl,
		}

		ip, err := utils.GetExtIp(c.SeedList[0])
		if err != nil {
			log.Fatal("get ext ip err:", err)
		}
		c.Domain = ip

		znd, err := znode.NewZnode(c)
		if err != nil {
			log.Fatal(err)
		}

		fmt.Printf("port: %d, wsport: %d, id: %x\n", p2p, ws, znd.Nnet.GetLocalNode().Id)

		znd.ApplyBytesReceived()
		znd.ApplyNeighborAdded()
		znd.ApplyNeighborRemoved()
		znd.ApplyVlcOnRelay()

		zebs = append(zebs, znd)
	}

	for i := 0; i < len(zebs); i++ {
		time.Sleep(112358 * time.Microsecond)

		err := zebs[i].Start(false)
		if err != nil {
			log.Fatal(err)
			return
		}

		err = zebs[i].Nnet.Join(s.Nnet.GetLocalNode().Addr)
		if err != nil {
			log.Fatal(err)
			return
		}
	}
}
