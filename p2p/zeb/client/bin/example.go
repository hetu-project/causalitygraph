package main

import (
	"encoding/hex"
	"fmt"
	"github.com/bufrr/net/util"
	"github.com/hetu-project/zeb/client"
	pb "github.com/hetu-project/zeb/protos"
	"google.golang.org/protobuf/proto"
	"log"
	"math/rand"
	"os"
	"strconv"
)

func main() {
	rpcServer := []string{"http://127.0.0.1:12345/rpc12345"}
	//rpcServer := []string{"http://192.168.1.110:13333/rpc13333"}

	client1 := client.NewClient([]byte("test1"), rpcServer)
	client2 := client.NewClient([]byte("test8"), rpcServer)
	err := client1.Connect()
	if err != nil {
		log.Fatal(err)
	}
	err = client2.Connect()
	if err != nil {
		log.Fatal(err)
	}

	addr2 := client2.Address()
	err = client1.Send(addr2, []byte("hello hetu!"+strconv.FormatInt(rand.Int63(), 10)), pb.ZType_Z_TYPE_ZCHAT)
	if err != nil {
		log.Fatal(err)
	}

	inboundMsg := <-client2.Receive

	msg := new(pb.InboundMsg)
	err = proto.Unmarshal(inboundMsg, msg)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("msg: ", string(msg.Data))
}

func randomMsg(to string) []byte {
	toBytes, _ := hex.DecodeString(to)
	id, _ := util.RandBytes(32)
	id2, _ := util.RandBytes(32)
	v := make(map[string]uint64)
	v[hex.EncodeToString(id)] = 1
	cc := pb.Clock{Values: v}

	ci := pb.ClockInfo{
		Clock:     &cc,
		NodeId:    id,
		MessageId: id2,
		Count:     2,
		CreateAt:  1243,
	}

	r := []byte("hello hetu! " + string(rune(os.Getpid())))
	chat := pb.ZChat{
		MessageData: r,
		Clock:       &ci,
	}
	d, err := proto.Marshal(&chat)
	if err != nil {
		log.Fatal(err)
	}

	zMsg := &pb.ZMessage{
		Data: d,
		To:   toBytes,
		Type: pb.ZType_Z_TYPE_ZCHAT,
	}

	data, err := proto.Marshal(zMsg)
	if err != nil {
		log.Fatal(err)
	}

	return data
}
