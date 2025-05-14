package znode

import (
	"testing"
)

func TestRpc(t *testing.T) {
	z := &Znode{}
	rs := NewRpcServer(z)
	rs.Start()
	defer rs.Stop()
	select {}
}
