package znode

import (
	"encoding/hex"
	"errors"
	"github.com/bufrr/net"
	"github.com/bufrr/net/node"
	"github.com/bufrr/net/overlay/chord"
	"github.com/bufrr/net/overlay/routing"
	protobuf "github.com/bufrr/net/protobuf"
	"github.com/bufrr/net/util"
	"github.com/hetu-project/zeb/config"
	"github.com/hetu-project/zeb/dht"
	pb "github.com/hetu-project/zeb/protos"
	"google.golang.org/protobuf/proto"
	"log"
	"math/rand"
	"net"
	"strconv"
	"sync"
	"time"
)

type NodeData struct {
	sync.RWMutex `json:"-"`
	StarTime     time.Time `json:"-"`

	RpcDomain string `json:"rpcDomain"`
	WsDomain  string `json:"wsDomain"`
	RpcPort   uint16 `json:"rpcPort"`
	WsPort    uint16 `json:"wsPort"`
	PublicKey []byte `json:"publicKey"`
}

func NewNodeData(domain string, rpcPort uint16, wsPort uint16) *NodeData {
	return &NodeData{
		RpcDomain: domain,
		WsDomain:  domain,
		RpcPort:   rpcPort,
		WsPort:    wsPort,
		StarTime:  time.Now(),
	}
}

type Znode struct {
	Neighbors map[string]*NodeData
	Nnet      *nnet.NNet
	keyPair   dht.KeyPair
	vlcConn   *net.UDPConn
	buf       [65536]byte
	config    config.Config
	msgBuffer map[string]chan *pb.InboundMsg
	cache     map[string]struct{}
}

func NewZnode(c config.Config) (*Znode, error) {
	nn, err := Create(c.Transport, c.P2pPort, c.Keypair.Id())
	if err != nil {
		return nil, err
	}
	udpAddr, err := net.ResolveUDPAddr("udp", c.VlcAddr)
	if err != nil {
		log.Fatal("ResolveUDPAddr err:", err)
	}

	conn, err := net.DialUDP("udp", nil, udpAddr)
	if err != nil {
		log.Fatal("DialUDP err:", err)
	}
	b := make([]byte, 65536)

	nd := &pb.NodeData{
		WebsocketPort: uint32(c.WsPort),
		JsonRpcPort:   uint32(c.RpcPort),
		Domain:        c.Domain,
	}

	nn.GetLocalNode().Node.Data, err = proto.Marshal(nd)
	if err != nil {
		log.Fatal(err)
	}

	neighbors := make(map[string]*NodeData)
	return &Znode{
		Nnet:      nn,
		keyPair:   c.Keypair,
		vlcConn:   conn,
		buf:       [65536]byte(b),
		config:    c,
		Neighbors: neighbors,
		msgBuffer: make(map[string]chan *pb.InboundMsg),
		cache:     make(map[string]struct{}),
	}, nil
}

func (z *Znode) GetConfig() *config.Config {
	return &z.config
}

func (z *Znode) Start(isCreate bool) error {
	wsServer := NewWsServer(z)
	go wsServer.Start()
	rpcServer := NewRpcServer(z)
	go rpcServer.Start()
	return z.Nnet.Start(isCreate)
}

func Create(transport string, port uint16, id []byte) (*nnet.NNet, error) {
	conf := &nnet.Config{
		Port:                  port,
		Transport:             transport,
		BaseStabilizeInterval: 500 * time.Millisecond,
	}

	nn, err := nnet.NewNNet(id, conf)
	if err != nil {
		return nil, err
	}

	return nn, nil
}

// readVlc reads a ZMessage byte array and performs the following steps:
// 1. Unmarshals the ZMessage into zm using protobuf
// 2. Creates a new Innermsg object and sets its Identity, Message, and Action fields
// 3. Marshals the Innermsg into message using protobuf
// 4. Calls the reqVlc method to send the message to VLC and receive the response
// 5. Unmarshals the response into innerMsg using protobuf
// 6. Marshals the Innermsg's Message field into response using protobuf
// 7. Returns the response and nil error if successful, otherwise returns nil response and the error encountered
// reqVlc takes a byte array b, writes the bytes to z.vlcConn, reads the response from z.vlcConn, and returns the response and error
func (z *Znode) readVlc(zMsg []byte, isClient bool) ([]byte, error) {
	zm := new(pb.ZMessage)
	err := proto.Unmarshal(zMsg, zm)
	if err != nil {
		return nil, err
	}

	id := hex.EncodeToString(zm.Id)
	if _, ok := z.cache[id]; ok {
		log.Printf("message already cached: %s", id)
		return zMsg, nil
	}
	z.cache[id] = struct{}{}

	innerMsg := new(pb.Innermsg)
	innerMsg.Identity = pb.Identity_IDENTITY_CLIENT
	if !isClient {
		innerMsg.Identity = pb.Identity_IDENTITY_SERVER
	}
	innerMsg.Message = zm
	innerMsg.Action = pb.Action_ACTION_WRITE

	msg, err := proto.Marshal(innerMsg)
	if err != nil {
		return nil, err
	}

	resp, err := z.reqVlc(msg)
	if err != nil {
		return nil, err
	}

	innerMsg = new(pb.Innermsg)
	err = proto.Unmarshal(resp, innerMsg)
	if err != nil {
		return nil, err
	}

	resp, err = proto.Marshal(innerMsg.Message)
	if err != nil {
		return nil, err
	}

	return resp, nil
}

func (z *Znode) reqVlc(b []byte) ([]byte, error) {
	_, err := z.vlcConn.Write(b)
	if err != nil {
		return nil, err
	}
	n, _, err := z.vlcConn.ReadFromUDP(z.buf[:])
	if err != nil {
		return nil, err
	}

	return z.buf[:n], nil
}

func (z *Znode) Id() string {
	return hex.EncodeToString(z.keyPair.Id())
}

func (z *Znode) handleWsZMsg(msg []byte) error {
	out := new(pb.OutboundMsg)
	err := proto.Unmarshal(msg, out)
	if err != nil {
		return err
	}

	d := out.Data
	if out.Type == pb.ZType_Z_TYPE_ZCHAT {
		msgId, _ := util.RandBytes(32)
		c, _ := z.getClock()
		pbc := pb.Clock{Values: c}

		ci := pb.ClockInfo{
			Clock:     &pbc,
			NodeId:    z.keyPair.Id(),
			MessageId: msgId,
			Count:     2,
			CreateAt:  1243,
		}

		chat := pb.ZChat{
			MessageData: out.Data,
			Clock:       &ci,
		}
		d, err = proto.Marshal(&chat)
		if err != nil {
			return err
		}
	}

	zMsg := &pb.ZMessage{
		Id:   out.Id,
		Data: d,
		From: out.From,
		To:   out.To,
		Type: out.Type,
	}

	b, _ := proto.Marshal(zMsg)
	zm := b

	if zMsg.Type == pb.ZType_Z_TYPE_ZCHAT {
		zm, err = z.readVlc(b, true)
		if err != nil {
			return err
		}
	}

	_, _, err = z.Nnet.SendBytesRelaySync(zm, zMsg.To)
	if err != nil {
		return err
	}
	return nil
}

func (z *Znode) getClock() (map[string]uint64, error) {
	v := rand.Int63()
	return map[string]uint64{
		z.Id(): uint64(v),
	}, nil
}

func (z *Znode) FindWsAddr(key []byte) (string, []byte, error) {
	c, ok := z.Nnet.Network.(*chord.Chord)
	if !ok {
		return "", nil, errors.New("overlay is not chord")
	}
	preds, err := c.FindPredecessors(key, 1)
	if err != nil {
		return "", nil, err
	}
	if len(preds) == 0 {
		return "", nil, errors.New("found no predecessors")
	}

	pred := preds[0]

	nd := new(pb.NodeData)
	err = proto.Unmarshal(pred.Data, nd)

	addr := "ws://" + nd.Domain + ":" + strconv.Itoa(int(nd.WebsocketPort))

	return addr, nd.PublicKey, nil
}

func (z *Znode) ApplyBytesReceived() {
	z.Nnet.MustApplyMiddleware(node.BytesReceived{Func: func(msg, msgID, srcID []byte, remoteNode *node.RemoteNode) ([]byte, bool) {
		zmsg := new(pb.ZMessage)
		err := proto.Unmarshal(msg, zmsg)
		if err != nil {
			log.Fatal(err)
		}

		_, err = z.Nnet.SendBytesRelayReply(msgID, []byte{}, srcID)
		if err != nil {
			log.Printf("SendBytesRelayReply err: %v\n", err)
		}

		id := hex.EncodeToString(zmsg.To)
		if _, ok := z.msgBuffer[id]; !ok {
			z.msgBuffer[id] = make(chan *pb.InboundMsg, 100)
		}

		data := zmsg.Data
		if zmsg.Type == pb.ZType_Z_TYPE_ZCHAT || zmsg.Type == pb.ZType_Z_TYPE_CLOCK {
			_, err = z.readVlc(msg, false)
			if err != nil {
				log.Fatal(err)
			}

			zc := new(pb.ZClock)
			err = proto.Unmarshal(zmsg.Data, zc)
			if err != nil {
				log.Printf("parse z msg err: %s", err)
				return nil, false
			}
			et := new(pb.EventTrigger)
			err = proto.Unmarshal(zc.Data, et)
			if err != nil {
				log.Printf("parse z clock err: %s", err)
				return nil, false
			}

			zchat := new(pb.ZChat)
			err = proto.Unmarshal(et.Message.Data, zchat)
			if err != nil {
				log.Printf("parse zchat err: %s", err)
				return nil, false
			}
			data = zchat.MessageData
		}

		inboundMsg := new(pb.InboundMsg)
		inboundMsg.Id = zmsg.Id
		inboundMsg.From = zmsg.From
		inboundMsg.Data = data

		z.msgBuffer[id] <- inboundMsg

		return msg, true
	}})
}

func (z *Znode) ApplyNeighborAdded() {
	z.Nnet.MustApplyMiddleware(chord.NeighborAdded{Func: func(remoteNode *node.RemoteNode, index int) bool {
		nd := new(pb.NodeData)
		err := proto.Unmarshal(remoteNode.Node.Data, nd)
		if err != nil {
			log.Printf("Unmarshal node data: %v\n", err)
			return false
		}
		neighbor := NewNodeData(nd.Domain, uint16(nd.JsonRpcPort), uint16(nd.WebsocketPort))
		z.Neighbors[string(remoteNode.Id)] = neighbor
		return true
	}})
}

func (z *Znode) ApplyNeighborRemoved() {
	z.Nnet.MustApplyMiddleware(chord.NeighborRemoved{Func: func(remoteNode *node.RemoteNode) bool {
		log.Printf("Neighbor %x removed", remoteNode.Id)
		delete(z.Neighbors, string(remoteNode.Id))
		return true
	}})
}

func (z *Znode) ApplyVlcOnRelay() {
	z.Nnet.MustApplyMiddleware(routing.RemoteMessageRouted{
		Func: func(message *node.RemoteMessage, localNode *node.LocalNode, nodes []*node.RemoteNode) (*node.RemoteMessage, *node.LocalNode, []*node.RemoteNode, bool) {
			if message.Msg.MessageType != protobuf.MessageType_BYTES {
				return message, localNode, nodes, false
			}

			b := new(protobuf.Bytes)
			err := proto.Unmarshal(message.Msg.Message, b)
			if err != nil {
				log.Printf("Unmarshal bytes err: %v\n", err)
				return message, localNode, nodes, false
			}

			zMsg := new(pb.ZMessage)
			err = proto.Unmarshal(b.Data, zMsg)
			if err != nil {
				log.Printf("Unmarshal zMsg err: %v\n", err)
				return message, localNode, nodes, false
			}

			if zMsg.Type != pb.ZType_Z_TYPE_ZCHAT && zMsg.Type != pb.ZType_Z_TYPE_CLOCK {
				//log.Printf("zMsg type is not Z_TYPE_ZCHAT or Z_TYPE_CLOCK\n")
				return message, localNode, nodes, false
			}

			resp, err := z.readVlc(b.Data, false)
			if err != nil {
				log.Printf("readVlc err: %v\n", err)
				return message, localNode, nodes, false
			}
			b.Data = resp
			message.Msg.Message, err = proto.Marshal(b)
			if err != nil {
				log.Printf("Marshal bytes err: %v\n", err)
				return message, localNode, nodes, false
			}

			log.Printf("Receive message %x at node %s from: %s to: %s\n", zMsg.Id, z.Id(), hex.EncodeToString(zMsg.From), hex.EncodeToString(zMsg.To))
			return message, localNode, nodes, true
		},
	})
}
