package examples

import (
	"github.com/hetu-project/zeb/client"
	pb "github.com/hetu-project/zeb/protos"
	"math/rand"
	"strconv"
	"testing"
	"time"
)

func init() {
	s := StartSeed()
	time.Sleep(1 * time.Second)
	StartCluster(s)

	time.Sleep(2 * time.Second)
}

func BenchmarkStartCluster(b *testing.B) {
	rpcServer := []string{"http://127.0.0.1:12345/rpc12345"}

	client1 := client.NewClient([]byte("test1"), rpcServer)
	client2 := client.NewClient([]byte("test8"), rpcServer)
	err := client1.Connect()
	if err != nil {
		b.Fatal(err)
	}
	err = client2.Connect()
	if err != nil {
		b.Fatal(err)
	}

	addr2 := client2.Address()

	go func() {
		for i := 0; i < b.N; i++ {
			<-client2.Receive
			if i%10000 == 0 {
				b.Log("received:", i)
			}
		}
	}()

	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		err = client1.Send(addr2, []byte("hello hetu!"+strconv.FormatInt(rand.Int63(), 10)), pb.ZType_Z_TYPE_RNG)
		if err != nil {
			b.Fatal(err)
		}
		if i%10000 == 0 {
			b.Log(i)
		}
	}
}
