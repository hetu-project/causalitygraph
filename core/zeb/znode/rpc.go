package znode

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	pb "github.com/hetu-project/zeb/protos"
	"google.golang.org/protobuf/proto"
	"io"
	"log"
	"net"
	"net/http"
	"strconv"
	"sync"
)

type Handler func(RpcServer, map[string]interface{}, context.Context) map[string]interface{}

type RpcServer struct {
	sync.RWMutex
	z            *Znode
	httpServer   *http.Server
	httpListener string
	timeout      int
	handlers     map[string]Handler
}

func NewRpcServer(z *Znode) *RpcServer {
	return &RpcServer{
		z:        z,
		handlers: make(map[string]Handler),
	}
}

func (z *Znode) startRpc() {
	rs := NewRpcServer(z)
	rs.Start()
}

func (rs *RpcServer) Start() error {
	rs.RegisterHandler("getWsAddr", getWsAddr)
	rs.RegisterHandler("getNodeStates", getNodeStates)
	rs.RegisterHandler("getNeighbors", getNeighbors)
	rs.RegisterHandler("getExtIp", getExtIp)
	rs.RegisterHandler("queryByKeyId", queryByKeyId)

	port := strconv.Itoa(int(rs.z.config.RpcPort))
	http.HandleFunc("/rpc"+port, rs.rpcHandler)
	rs.httpListener = "0.0.0.0:" + port
	rs.httpServer = &http.Server{
		Addr: rs.httpListener,
	}
	go rs.httpServer.ListenAndServe()
	return nil
}

func (rs *RpcServer) rpcHandler(w http.ResponseWriter, r *http.Request) {
	rs.RLock()
	defer rs.RUnlock()

	w.Header().Add("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept")
	w.Header().Set("content-type", "application/json;charset=utf-8")
	w.Header().Set("Access-Control-Allow-Origin", "*")

	if r.Method != "POST" {
		w.Write([]byte("POST method is required"))
		return
	}

	body, err := io.ReadAll(r.Body)
	defer r.Body.Close()
	if err != nil {
		w.Write([]byte(fmt.Sprintf(`{"error": "%v"}`, err)))
		return
	}

	req := make(map[string]interface{})
	err = json.Unmarshal(body, &req)
	if err != nil {
		w.Write([]byte(fmt.Sprintf(`{"error": "%v"}`, err)))
		return
	}

	if req["method"] == nil {
		w.Write([]byte(`{"error": "method is required"}`))
		return
	}

	req["remoteAddr"] = r.RemoteAddr[:len(r.RemoteAddr)-5]

	method := req["method"].(string)
	handler, ok := rs.handlers[method]
	if !ok {
		w.Write([]byte(fmt.Sprintf(`{"error": "method %s not found"}`, method)))
		return
	}
	data, err := json.Marshal(handler(*rs, req, r.Context()))
	if err != nil {
		w.Write([]byte(fmt.Sprintf(`{"error": "%v"}`, err)))
		return
	}
	w.Write(data)
}

func (rs *RpcServer) Stop() {
	rs.Lock()
	defer rs.Unlock()
	rs.httpServer.Close()
}

func (rs *RpcServer) RegisterHandler(method string, handler Handler) {
	rs.Lock()
	defer rs.Unlock()

	rs.handlers[method] = handler
}

func getWsAddr(rs RpcServer, params map[string]interface{}, ctx context.Context) map[string]interface{} {
	if len(params) == 0 {
		return map[string]interface{}{
			"error": "address1 is required",
		}
	}

	address, ok := params["address"].(string)
	if !ok {
		return map[string]interface{}{
			"error": "address2 is required",
		}
	}
	b, err := hex.DecodeString(address)
	if err != nil {
		return map[string]interface{}{
			"error": err.Error(),
		}
	}

	addr, _, err := rs.z.FindWsAddr(b)
	return map[string]interface{}{
		"wsAddr": addr,
	}
}

func getNodeStates(rs RpcServer, params map[string]interface{}, ctx context.Context) map[string]interface{} {
	res := make(map[string]interface{})
	res["id"] = hex.EncodeToString(rs.z.Nnet.GetLocalNode().Id)
	res["addr"] = rs.z.config.Domain
	res["p2pPort"] = rs.z.config.P2pPort
	res["rpcPort"] = rs.z.config.RpcPort
	res["wsPort"] = rs.z.config.WsPort

	return res
}

func getNeighbors(rs RpcServer, params map[string]interface{}, ctx context.Context) map[string]interface{} {
	res := make(map[string]interface{})
	for id, v := range rs.z.Neighbors {
		h := hex.EncodeToString([]byte(id))
		res[h] = v
	}
	return res
}

func getExtIp(rs RpcServer, params map[string]interface{}, ctx context.Context) map[string]interface{} {
	addr, ok := params["remoteAddr"].(string)
	if !ok {
		return map[string]interface{}{
			"error": "remoteAddr is required",
		}
	}
	host, _, err := net.SplitHostPort(addr)
	if err != nil {
		return map[string]interface{}{
			"error": err.Error(),
		}
	}
	return map[string]interface{}{
		"extIp": host,
	}
}

func queryByKeyId(rs RpcServer, params map[string]interface{}, ctx context.Context) map[string]interface{} {
	gwt, ok := params["gatewayType"].(float64)
	if !ok {
		return map[string]interface{}{
			"error": "gatewayType is required",
		}
	}

	id, ok := params["id"].(string)
	if !ok {
		return map[string]interface{}{
			"error": "id is required",
		}
	}

	index, ok := params["index"].(float64)
	if !ok {
		index = 0
	}
	q, _ := proto.Marshal(&pb.QueryByTableKeyID{LastPos: uint64(index)})

	gateway := pb.ZGateway{
		Type:      pb.GatewayType(gwt),
		Method:    pb.QueryMethod_QUERY_BY_TABLE_KEYID,
		Data:      q,
		RequestId: id,
	}

	gw, _ := proto.Marshal(&gateway)
	p2pMsg := pb.ZMessage{
		Type: pb.ZType_Z_TYPE_GATEWAY,
		Data: gw,
	}

	innerMsg := pb.Innermsg{
		Identity: pb.Identity_IDENTITY_CLIENT,
		Action:   pb.Action_ACTION_READ,
		Message:  &p2pMsg,
	}

	data, _ := proto.Marshal(&innerMsg)

	log.Printf("queryByKeyId: %s", hex.EncodeToString(data))

	resp, err := rs.z.reqVlc(data)
	if err != nil {
		return nil
	}

	log.Printf("queryByKeyId resp: %s", hex.EncodeToString(resp))

	inner := new(pb.Innermsg)
	err = proto.Unmarshal(resp, inner)
	if err != nil {
		return nil
	}

	return map[string]interface{}{
		"result": hex.EncodeToString(inner.Message.Data),
	}
}
