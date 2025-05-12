package znode

import (
	"encoding/hex"
	"github.com/gorilla/websocket"
	pb "github.com/hetu-project/zeb/protos"
	"google.golang.org/protobuf/proto"
	"log"
	"net"
	"net/http"
	"strconv"
	"sync"
)

type WsServer struct {
	sync.Mutex
	listener net.Listener
	server   *http.Server
	handlers map[string]Handler
	z        *Znode
	clients  map[string]*websocket.Conn
}

func NewWsServer(z *Znode) *WsServer {
	return &WsServer{
		z:        z,
		handlers: make(map[string]Handler),
		clients:  make(map[string]*websocket.Conn),
	}
}

func (ws *WsServer) Start() {
	port := strconv.Itoa(int(ws.z.config.WsPort))
	http.HandleFunc("/ws"+port, ws.vlcHandler)
	http.ListenAndServe("0.0.0.0:"+port, nil)
}

func (ws *WsServer) vlcHandler(w http.ResponseWriter, r *http.Request) {
	var upgrader = websocket.Upgrader{
		CheckOrigin: func(r *http.Request) bool {
			return true
		},
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Print("upgrade:", err)
		return
	}
	defer conn.Close()
	_, message, err := conn.ReadMessage()
	if err != nil {
		log.Println("ws read id err: ", err)
	}
	id := hex.EncodeToString(message)
	ws.clients[id] = conn
	ws.z.msgBuffer[id] = make(chan *pb.InboundMsg, 100)

	go func() {
		for {
			_, m, err := conn.ReadMessage()
			if err != nil {
				log.Println("ws read err:", err)
				return
			}

			err = ws.z.handleWsZMsg(m)
			if err != nil {
				log.Printf("handle ws msg err: %s", err)
			}
		}
	}()

	go func() {
		for {
			select {
			case inboundMsg := <-ws.z.msgBuffer[id]:
				msg, err := proto.Marshal(inboundMsg)
				if err != nil {
					log.Printf("marshal inbound msg err: %s", err)
				}
				err = conn.WriteMessage(websocket.BinaryMessage, msg)
				if err != nil {
					log.Println("ws write err:", err)
					return
				}
			}
		}
	}()

	select {}
}
