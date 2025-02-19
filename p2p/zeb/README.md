# zeb

zeb is a p2p relay network with verifiable VLC (virtual logic clock) causal order.
This system is currently in poc stage.

## Install

### build from source

```shell
git clone https://github.com/hetu-project/zeb.git
cd zeb
make

./build/zeb -h
```

Run node with create mode:
`go run main.go --id Hello0 --domain 127.0.0.1`

Run node with join mode:
`go run main.go --remote tcp://127.0.0.1:33333 --id Hello0 --p2p 33334 --ws 23334`

# How to test

```
go run examples/bin/basic.go
```

# Benchmark

```shell
cd example
go test -bench='^\QBenchmarkStartCluster\E$'
```

