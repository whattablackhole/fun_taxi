package main

import (
	"context"
	"errors"
	"flag"
	"os"
	"time"

	"github.com/envoyproxy/go-control-plane/pkg/cache/v3"
	"github.com/envoyproxy/go-control-plane/pkg/server/v3"
	"github.com/envoyproxy/go-control-plane/pkg/test/v3"

	capi "github.com/hashicorp/consul/api"
)

var (
	l    Logger
	port uint
)

func init() {
	l = Logger{}

	flag.BoolVar(&l.Debug, "debug", false, "Enable xDS server debug logging")

	flag.UintVar(&port, "port", 18000, "xDS management server port")

}

type EndpointUpdate struct {
	ServiceName string
	Endpoints   []*capi.ServiceEntry
}

func main() {
	client, err := capi.NewClient(&capi.Config{Address: os.Getenv("CONSUL_URL")})
	if err != nil {
		panic(err)
	}

	cache := cache.NewSnapshotCache(false, cache.IDHash{}, l)

	srv := server.NewServer(context.Background(), cache, &test.Callbacks{Debug: l.Debug})

	go RunServer(srv, port)

	endpointUpdates := make(chan *EndpointUpdate, 10)

	// map[serviceName] -> cancelFunction
	managedWatchers := make(map[string]context.CancelFunc)

	go func() {
		endpointState := make(map[string][]*capi.ServiceEntry)

		for update := range endpointUpdates {
			l.Infof("Received endpoint update for %s", update.ServiceName)
			endpointState[update.ServiceName] = update.Endpoints

			updateSnapshotCache(cache, "envoy-node-1.dc1", endpointState)
		}
	}()

	var servicesLastIndex uint64

	for {
		serviceEntries, meta, err := client.Catalog().Services(&capi.QueryOptions{WaitIndex: servicesLastIndex})

		if err != nil {
			l.Errorf("error fetching service list: %v", err)
			time.Sleep(5 * time.Second)
			continue
		}

		servicesLastIndex = meta.LastIndex

		currentServices := make(map[string]bool)
		for name := range serviceEntries {
			currentServices[name] = true
		}

		for name, cancel := range managedWatchers {
			if !currentServices[name] {
				l.Infof("Service %s removed, stopping its watcher.", name)
				cancel()
				delete(managedWatchers, name)
			}
		}

		for name := range currentServices {
			if _, exists := managedWatchers[name]; !exists {
				l.Infof("New service %s detected, starting its watcher.", name)
				ctx, cancel := context.WithCancel(context.Background())
				managedWatchers[name] = cancel
				go WatchConsulService(ctx, name, client, endpointUpdates)
			}
		}
	}
}
func WatchConsulService(ctx context.Context, serviceName string, client *capi.Client, updates chan<- *EndpointUpdate) {
	var lastIndex uint64
	for {
		select {
		case <-ctx.Done():
			l.Infof("Stopping watch for service %s", serviceName)
			return
		default:
		}

		queryOpts := &capi.QueryOptions{
			WaitIndex: lastIndex,
			WaitTime:  5 * time.Minute,
		}

		entries, meta, err := client.Health().Service(serviceName, "", true, queryOpts.WithContext(ctx))
		if err != nil {
			if errors.Is(err, context.Canceled) {
				l.Infof("Watch for service %s cancelled during query.", serviceName)
				return
			}
			l.Infof("Error watching Consul service %s: %v", serviceName, err)
			time.Sleep(5 * time.Second)
			continue
		}

		if meta != nil && meta.LastIndex == lastIndex {
			continue
		}

		lastIndex = meta.LastIndex
		l.Infof("Detected update for service %s at index %d. Found %d instances.", serviceName, meta.LastIndex, len(entries))

		updates <- &EndpointUpdate{
			ServiceName: serviceName,
			Endpoints:   entries,
		}
	}
}
