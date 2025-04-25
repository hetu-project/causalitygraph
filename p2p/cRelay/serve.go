package main

import (
	//"bufio"
	"context"
	"fmt"

	//"math"
	//"os"
	"time"

	"github.com/bep/debounce"
	"github.com/fatih/color"

	//"github.com/fiatjaf/eventstore"
	//"github.com/fiatjaf/eventstore/mysql_etcd"
	//"github.com/fiatjaf/eventstore/slicestore"
	"github.com/fiatjaf/khatru"
	// dbinit "github.com/maoaixiao1314/orbitdb/internal/db"
	orbitdb "github.com/maoaixiao1314/orbitdb/orbitdb"
	"github.com/nbd-wtf/go-nostr"
	"github.com/urfave/cli/v3"
)

var serve = &cli.Command{
	Name:                      "serve",
	Usage:                     "starts an in-memory relay for testing purposes",
	DisableSliceFlagSeparator: true,
	Flags: []cli.Flag{
		&cli.StringFlag{
			Name:  "hostname",
			Usage: "hostname where to listen for connections",
			Value: "localhost",
		},
		&cli.UintFlag{
			Name:  "port",
			Usage: "port where to listen for connections",
			Value: 10547,
		},
		&cli.StringFlag{
			Name:        "events",
			Usage:       "file containing the initial batch of events that will be served by the relay as newline-separated JSON (jsonl)",
			DefaultText: "the relay will start empty",
		},
		// &cli.StringFlag{
		// 	Name:  "dburl",
		// 	Usage: "URL of the database to use",
		// },

		// &cli.StringFlag{
		// 	Name:  "etcdurl",
		// 	Usage: "URL of the etcd server to use",
		// },
	},
	Action: func(ctx context.Context, c *cli.Command) error {

		// dbUrl := c.String("dburl")
		// etcdUrl := c.String("etcdurl")

		// 初始化数据库
		if err := orbitdb.Init(); err != nil {
			return fmt.Errorf("初始化数据库失败: %w", err)
		}
		defer orbitdb.Close()

		// 获取 OrbitDB 存储实例
		store, err := orbitdb.GetStore()
		if err != nil {
			return fmt.Errorf("获取存储实例失败: %w", err)
		}

		// 创建适配器
		adapter := orbitdb.NewOrbitDBAdapter(store)

		rl := khatru.NewRelay()

		rl.Info.Name = "nak serve"
		rl.Info.Description = "a local relay for testing, debugging and development."
		rl.Info.Software = "https://github.com/fiatjaf/nak"
		rl.Info.Version = version

		rl.QueryEvents = append(rl.QueryEvents, adapter.QueryEvents)
		// rl.CountEvents = append(rl.CountEvents, db.(eventstore.Counter).CountEvents)
		rl.DeleteEvent = append(rl.DeleteEvent, adapter.DeleteEvent)
		rl.StoreEvent = append(rl.StoreEvent, adapter.SaveEvent)

		started := make(chan bool)
		exited := make(chan error)

		hostname := c.String("hostname")
		port := int(c.Uint("port"))

		go func() {
			err := rl.Start(hostname, port, started)
			exited <- err
		}()

		var printStatus func()

		// relay logging
		rl.RejectFilter = append(rl.RejectFilter, func(ctx context.Context, filter nostr.Filter) (reject bool, msg string) {
			log("    got %s %v\n", color.HiYellowString("request"), colors.italic(filter))
			printStatus()
			return false, ""
		})
		rl.RejectCountFilter = append(rl.RejectCountFilter, func(ctx context.Context, filter nostr.Filter) (reject bool, msg string) {
			log("    got %s %v\n", color.HiCyanString("count request"), colors.italic(filter))
			printStatus()
			return false, ""
		})
		rl.RejectEvent = append(rl.RejectEvent, func(ctx context.Context, event *nostr.Event) (reject bool, msg string) {
			log("    got %s %v\n", color.BlueString("event"), colors.italic(event))
			printStatus()
			return false, ""
		})

		d := debounce.New(time.Second * 2)
		printStatus = func() {
			d(func() {
				totalEvents := 0
				ch, _ := adapter.QueryEvents(ctx, nostr.Filter{})
				for range ch {
					totalEvents++
				}
				subs := rl.GetListeningFilters()

				log("  %s events stored: %s, subscriptions opened: %s\n", color.HiMagentaString("•"), color.HiMagentaString("%d", totalEvents), color.HiMagentaString("%d", len(subs)))
			})
		}

		<-started
		log("%s relay running at %s\n", color.HiRedString(">"), colors.boldf("ws://%s:%d", hostname, port))

		return <-exited
	},
}
