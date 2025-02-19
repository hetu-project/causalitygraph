package client

import (
	"encoding/hex"
	"errors"
	"github.com/bufrr/net/util"
	"github.com/gorilla/websocket"
	"github.com/hetu-project/zeb/dht"
	pb "github.com/hetu-project/zeb/protos"
	"github.com/hetu-project/zeb/utils"
	"golang.org/x/crypto/sha3"
	"google.golang.org/protobuf/proto"
	"log"
	"net"
	"net/url"
)

type Config struct {
	SeedRpcServer []string
}

func NewClientConfig(rpcServer []string) *Config {
	return &Config{
		SeedRpcServer: rpcServer,
	}
}

type Client struct {
	conn    *websocket.Conn
	key     dht.KeyPair
	Receive chan []byte
	config  *Config
}

func (c *Client) Send(address string, data []byte, zType pb.ZType) error {
	if c.conn == nil {
		return errors.New("ws not connected")
	}
	outMsg := new(pb.OutboundMsg)
	outMsg.From = c.key.Id()
	to, err := hex.DecodeString(address)
	if err != nil {
		return err
	}
	outMsg.To = to
	outMsg.Data = data
	outMsg.Id, _ = util.RandBytes(32)
	outMsg.Type = zType
	m, _ := proto.Marshal(outMsg)
	err = c.conn.WriteMessage(websocket.BinaryMessage, m)
	if err != nil {
		return err
	}
	return nil
}

func NewClient(seed []byte, rpcServer []string) *Client {
	h := sha3.New256().Sum(seed)
	keypair, _ := dht.GenerateKeyPair(h[:32])
	recv := make(chan []byte)
	c := NewClientConfig(rpcServer)

	return &Client{nil, keypair, recv, c}
}

func (c *Client) Connect() error {
	wsAddr, err := c.GetWsAddr()
	if err != nil {
		return err
	}
	return c.connect(wsAddr)
}

func (c *Client) connect(addr string) error {

	u, _ := url.Parse(addr)

	_, port, _ := net.SplitHostPort(u.Host)

	conn, _, err := websocket.DefaultDialer.Dial(addr+"/ws"+port, nil)
	if err != nil {
		return err
	}
	c.conn = conn
	err = conn.WriteMessage(websocket.TextMessage, c.key.Id())
	if err != nil {
		return err
	}

	go c.readMsg()
	log.Println("connected to", addr)

	return nil
}

func (c *Client) readMsg() {
	for {
		_, msg, err := c.conn.ReadMessage()
		if err != nil {
			log.Println("read conn err: ", err)
			return
		}
		c.Receive <- msg
	}
}

func (c *Client) Address() string {
	return hex.EncodeToString(c.key.Id())
}

func (c *Client) GetWsAddr() (string, error) {
	wsAddr, err := utils.GetWsAddr(c.config.SeedRpcServer[0], c.Address())
	if err != nil {
		return "", err
	}
	return wsAddr, nil
}
