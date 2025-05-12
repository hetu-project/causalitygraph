package main

import (
	"github.com/hetu-project/zeb/examples"
	"time"
)

func main() {
	s := examples.StartSeed()
	time.Sleep(3 * time.Second)

	examples.StartCluster(s)

	select {}
}
