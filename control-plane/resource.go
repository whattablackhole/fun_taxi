package main

import (
	"context"
	"log"
	"net"
	"strconv"
	"time"

	accesslogv3 "github.com/envoyproxy/go-control-plane/envoy/config/accesslog/v3"
	cluster "github.com/envoyproxy/go-control-plane/envoy/config/cluster/v3"
	core "github.com/envoyproxy/go-control-plane/envoy/config/core/v3"
	endpoint "github.com/envoyproxy/go-control-plane/envoy/config/endpoint/v3"
	listener "github.com/envoyproxy/go-control-plane/envoy/config/listener/v3"
	route "github.com/envoyproxy/go-control-plane/envoy/config/route/v3"
	fileaccesslog "github.com/envoyproxy/go-control-plane/envoy/extensions/access_loggers/file/v3"
	router "github.com/envoyproxy/go-control-plane/envoy/extensions/filters/http/router/v3"
	hcm "github.com/envoyproxy/go-control-plane/envoy/extensions/filters/network/http_connection_manager/v3"
	tlsv3 "github.com/envoyproxy/go-control-plane/envoy/extensions/transport_sockets/tls/v3"
	upstreamv3 "github.com/envoyproxy/go-control-plane/envoy/extensions/upstreams/http/v3"
	"github.com/envoyproxy/go-control-plane/pkg/cache/types"
	"github.com/envoyproxy/go-control-plane/pkg/cache/v3"
	"github.com/envoyproxy/go-control-plane/pkg/resource/v3"
	"github.com/envoyproxy/go-control-plane/pkg/wellknown"
	capi "github.com/hashicorp/consul/api"
	"google.golang.org/protobuf/types/known/anypb"
	"google.golang.org/protobuf/types/known/durationpb"
	"google.golang.org/protobuf/types/known/structpb"
	"google.golang.org/protobuf/types/known/wrapperspb"
	"sigs.k8s.io/yaml"
)

const (
	RouteName    = "streamgate_route"
	ListenerName = "streamgate_listener"
	ListenerPort = 10000
)

func makeEdsCluster(clusterName string, useTls bool) *cluster.Cluster {
	var ringConfig = &cluster.Cluster_RingHashLbConfig{
		MinimumRingSize: &wrapperspb.UInt64Value{Value: 1024},
		HashFunction:    cluster.Cluster_RingHashLbConfig_XX_HASH,
		MaximumRingSize: &wrapperspb.UInt64Value{Value: 4096},
	}

	clusterConfig := &cluster.Cluster{
		Name:                 clusterName,
		ConnectTimeout:       durationpb.New(5 * time.Second),
		ClusterDiscoveryType: &cluster.Cluster_Type{Type: cluster.Cluster_EDS},
		EdsClusterConfig: &cluster.Cluster_EdsClusterConfig{
			EdsConfig: &core.ConfigSource{
				ConfigSourceSpecifier: &core.ConfigSource_ApiConfigSource{
					ApiConfigSource: &core.ApiConfigSource{
						ApiType:             core.ApiConfigSource_GRPC,
						TransportApiVersion: core.ApiVersion_V3,
						GrpcServices: []*core.GrpcService{
							{
								TargetSpecifier: &core.GrpcService_EnvoyGrpc_{
									EnvoyGrpc: &core.GrpcService_EnvoyGrpc{
										ClusterName: "xds_cluster",
									},
								},
							},
						},
					},
				},
			},
			ServiceName: clusterName,
		},
		OutlierDetection: &cluster.OutlierDetection{},
		LbPolicy:         cluster.Cluster_RING_HASH,
		LbConfig:         &cluster.Cluster_RingHashLbConfig_{RingHashLbConfig: ringConfig},
	}

	if useTls {
		tlsContext := &tlsv3.UpstreamTlsContext{
			CommonTlsContext: &tlsv3.CommonTlsContext{
				AlpnProtocols: []string{"h2", "http/1.1"},
				ValidationContextType: &tlsv3.CommonTlsContext_ValidationContext{
					ValidationContext: &tlsv3.CertificateValidationContext{
						TrustedCa: &core.DataSource{
							Specifier: &core.DataSource_Filename{
								Filename: "/etc/envoy/certs/ca.crt",
							},
						},
					},
				},
			},
			Sni: clusterName,
		}

		tlsContextAny, err := anypb.New(tlsContext)
		if err != nil {
			panic(err)
		}

		clusterConfig.TransportSocket = &core.TransportSocket{
			Name: "envoy.transport_sockets.tls",
			ConfigType: &core.TransportSocket_TypedConfig{
				TypedConfig: tlsContextAny,
			},
		}

		httpProtocolOptions := &upstreamv3.HttpProtocolOptions{
			UpstreamProtocolOptions: &upstreamv3.HttpProtocolOptions_AutoConfig{
				AutoConfig: &upstreamv3.HttpProtocolOptions_AutoHttpConfig{
					Http2ProtocolOptions: &core.Http2ProtocolOptions{},
					HttpProtocolOptions:  &core.Http1ProtocolOptions{},
				},
			},
		}

		pbst, err := anypb.New(httpProtocolOptions)
		if err != nil {
			panic(err)
		}

		clusterConfig.TypedExtensionProtocolOptions = map[string]*anypb.Any{
			"envoy.extensions.upstreams.http.v3.HttpProtocolOptions": pbst,
		}
	}

	return clusterConfig
}

func createEdsResponse(clusterName string, serviceEntries []*capi.ServiceEntry) (types.Resource, error) {
	lbEndpoints := make([]*endpoint.LbEndpoint, 0, len(serviceEntries))

	for _, entry := range serviceEntries {
		addr := entry.Service.Address
		if addr == "" {
			addr = entry.Node.Address
		}

		if net.ParseIP(addr) == nil {
			ips, err := net.LookupIP(addr)
			if err != nil || len(ips) == 0 {
				log.Printf("DNS lookup failed for hostname '%s' within Docker network: %v. Skipping endpoint.", addr, err)
				continue
			}
			addr = ips[0].String()
			log.Printf("Resolved service name '%s' to IP '%s'", entry.Service.Address, addr)
		}

		lbEp := &endpoint.LbEndpoint{
			HostIdentifier: &endpoint.LbEndpoint_Endpoint{
				Endpoint: &endpoint.Endpoint{
					Address: &core.Address{
						Address: &core.Address_SocketAddress{
							SocketAddress: &core.SocketAddress{
								Protocol: core.SocketAddress_TCP,
								Address:  addr,
								PortSpecifier: &core.SocketAddress_PortValue{
									PortValue: uint32(entry.Service.Port),
								},
							},
						},
					},
				},
			},
			HealthStatus: core.HealthStatus_HEALTHY,
		}
		lbEndpoints = append(lbEndpoints, lbEp)
	}

	cla := &endpoint.ClusterLoadAssignment{
		ClusterName: clusterName,
		Endpoints: []*endpoint.LocalityLbEndpoints{{
			Locality: &core.Locality{
				Region: "default-region",
				Zone:   "default-zone",
			},
			LbEndpoints: lbEndpoints,
		}},
	}

	return cla, nil
}

func makeRoute(routeName string) *route.RouteConfiguration {
	return &route.RouteConfiguration{
		Name: routeName,
		VirtualHosts: []*route.VirtualHost{{
			Name:    "streamgate",
			Domains: []string{"*"},
			Routes: []*route.Route{
				{
					Match: &route.RouteMatch{
						PathSpecifier: &route.RouteMatch_Prefix{
							Prefix: "/streamgate/ws",
						},
					},
					Action: &route.Route_Route{
						Route: &route.RouteAction{
							ClusterSpecifier: &route.RouteAction_Cluster{
								Cluster: "streamgate",
							},

							HashPolicy: []*route.RouteAction_HashPolicy{
								{
									PolicySpecifier: &route.RouteAction_HashPolicy_QueryParameter_{

										QueryParameter: &route.RouteAction_HashPolicy_QueryParameter{
											Name: "user-id",
										},
									},
									Terminal: true,
								},
							},
							PrefixRewrite: "/ws",
							HostRewriteSpecifier: &route.RouteAction_HostRewriteLiteral{
								HostRewriteLiteral: "streamgate",
							},

							UpgradeConfigs: []*route.RouteAction_UpgradeConfig{
								{
									UpgradeType: "Websocket",
								},
							},

							IdleTimeout: durationpb.New(0),
						},
					},
				},
				{
					Match: &route.RouteMatch{
						PathSpecifier: &route.RouteMatch_Path{
							Path: "/streamgate/health",
						},
					},
					Action: &route.Route_Route{
						Route: &route.RouteAction{
							ClusterSpecifier: &route.RouteAction_Cluster{
								Cluster: "streamgate",
							},
							PrefixRewrite: "/health",
						},
					},
				},

				{
					Match: &route.RouteMatch{
						PathSpecifier: &route.RouteMatch_Prefix{
							Prefix: "/",
						},
					},
					Action: &route.Route_Route{
						Route: &route.RouteAction{
							ClusterSpecifier: &route.RouteAction_Cluster{
								Cluster: "streamgate",
							},
							HashPolicy: []*route.RouteAction_HashPolicy{
								{
									PolicySpecifier: &route.RouteAction_HashPolicy_Header_{
										Header: &route.RouteAction_HashPolicy_Header{
											HeaderName: "x-user-id",
										},
									},
									Terminal: true,
								},
							},
						},
					},
				},
			},
		}},
	}
}
func makeHTTPListener(listenerName, route string) *listener.Listener {
	accessLogConfig := &fileaccesslog.FileAccessLog{
		Path: "/dev/stdout",
		AccessLogFormat: &fileaccesslog.FileAccessLog_LogFormat{
			LogFormat: &core.SubstitutionFormatString{
				Format: &core.SubstitutionFormatString_JsonFormat{
					JsonFormat: &structpb.Struct{
						Fields: map[string]*structpb.Value{
							"start_time":     structpb.NewStringValue("%START_TIME%"),
							"method":         structpb.NewStringValue("%REQ(:METHOD)%"),
							"path":           structpb.NewStringValue("%REQ(X-ENVOY-ORIGINAL-PATH?:PATH)%"),
							"protocol":       structpb.NewStringValue("%PROTOCOL%"),
							"response_code":  structpb.NewStringValue("%RESPONSE_CODE%"),
							"response_flags": structpb.NewStringValue("%RESPONSE_FLAGS%"),
							"bytes_received": structpb.NewStringValue("%BYTES_RECEIVED%"),
							"bytes_sent":     structpb.NewStringValue("%BYTES_SENT%"),
							"duration_ms":    structpb.NewStringValue("%DURATION%"),
							"upstream_host":  structpb.NewStringValue("%UPSTREAM_HOST%"),
							"user_agent":     structpb.NewStringValue("%REQ(USER-AGENT)%"),
							"request_id":     structpb.NewStringValue("%REQ(X-REQUEST-ID)%"),
							"authority":      structpb.NewStringValue("%REQ(:AUTHORITY)%"),
							"user_id_header": structpb.NewStringValue("%REQ(X-USER-ID)%"),
							"user_id_query":  structpb.NewStringValue("%REQ(user-id?QUERY)%"),
							"computed_hash":  structpb.NewStringValue("%FILTER_STATE(envoy.lb.computed_hash)%"),
						},
					},
				},
			},
		},
	}

	accessLogTypedConfig, err := anypb.New(accessLogConfig)
	if err != nil {
		log.Fatalf("failed to marshal access log config: %v", err)
	}
	accessLogs := []*accesslogv3.AccessLog{
		{
			Name: wellknown.FileAccessLog,
			ConfigType: &accesslogv3.AccessLog_TypedConfig{
				TypedConfig: accessLogTypedConfig,
			},
		},
	}
	routerConfig, _ := anypb.New(&router.Router{})
	manager := &hcm.HttpConnectionManager{
		AccessLog:  accessLogs,
		CodecType:  hcm.HttpConnectionManager_AUTO,
		StatPrefix: "http",
		RouteSpecifier: &hcm.HttpConnectionManager_Rds{
			Rds: &hcm.Rds{
				ConfigSource:    makeConfigSource(),
				RouteConfigName: route,
			},
		},
		HttpFilters: []*hcm.HttpFilter{{
			Name:       wellknown.Router,
			ConfigType: &hcm.HttpFilter_TypedConfig{TypedConfig: routerConfig},
		}},
		UpgradeConfigs: []*hcm.HttpConnectionManager_UpgradeConfig{
			{
				UpgradeType: "h2c",
			},
		},
	}

	pbst, err := anypb.New(manager)
	if err != nil {
		panic(err)
	}

	lst := &listener.Listener{
		Name: listenerName,
		Address: &core.Address{
			Address: &core.Address_SocketAddress{
				SocketAddress: &core.SocketAddress{
					Protocol: core.SocketAddress_TCP,
					Address:  "0.0.0.0",
					PortSpecifier: &core.SocketAddress_PortValue{
						PortValue: ListenerPort,
					},
				},
			},
		},
	}

	filterChain := &listener.FilterChain{
		Filters: []*listener.Filter{{
			Name: wellknown.HTTPConnectionManager,
			ConfigType: &listener.Filter_TypedConfig{
				TypedConfig: pbst,
			},
		}},
	}

	lst.FilterChains = []*listener.FilterChain{filterChain}
	return lst
}

func makeConfigSource() *core.ConfigSource {
	source := &core.ConfigSource{}
	source.ResourceApiVersion = resource.DefaultAPIVersion
	source.ConfigSourceSpecifier = &core.ConfigSource_ApiConfigSource{
		ApiConfigSource: &core.ApiConfigSource{
			TransportApiVersion:       resource.DefaultAPIVersion,
			ApiType:                   core.ApiConfigSource_GRPC,
			SetNodeOnFirstMessageOnly: true,
			GrpcServices: []*core.GrpcService{{
				TargetSpecifier: &core.GrpcService_EnvoyGrpc_{
					EnvoyGrpc: &core.GrpcService_EnvoyGrpc{ClusterName: "xds_cluster"},
				},
			}},
		},
	}
	return source
}

func updateSnapshotCache(snapshotCache cache.SnapshotCache, nodeId string, entriesState map[string][]*capi.ServiceEntry) {
	clusters := []types.Resource{}
	endpoints := []types.Resource{}

	for serviceName, healthyEntries := range entriesState {

		if serviceName == "streamgate" {
			clusters = append(clusters, makeEdsCluster(serviceName, true))
		} else {
			clusters = append(clusters, makeEdsCluster(serviceName, false))
		}

		edsResource, err := createEdsResponse(serviceName, healthyEntries)
		if err != nil {
			l.Errorf("eds creation error: %v", err)
			continue
		}
		endpoints = append(endpoints, edsResource)
	}
	version := strconv.FormatInt(time.Now().Unix(), 10)

	snapshot, err := cache.NewSnapshot(
		version,
		map[resource.Type][]types.Resource{
			resource.ClusterType:  clusters,
			resource.EndpointType: endpoints,
			resource.RouteType:    {makeRoute(RouteName)},
			resource.ListenerType: {makeHTTPListener(ListenerName, RouteName)},
		},
	)
	if err := snapshot.Consistent(); err != nil {
		l.Errorf("snapshot creation error: %v", err)
		DebugSnapshot(snapshot)
		return
	}

	if err != nil {
		l.Errorf("snapshot creation error: %v", err)
		return
	}

	if err := snapshotCache.SetSnapshot(context.Background(), nodeId, snapshot); err != nil {
		l.Errorf("snapshot set error: %v", err)
	}
}

func DebugSnapshot(snap *cache.Snapshot) {
	log.Println("--- Debugging Snapshot ---")

	for _, resType := range []resource.Type{resource.ListenerType, resource.ClusterType, resource.RouteType} {
		log.Printf("====== RESOURCES FOR TYPE: %s ======\n", resType)

		resources := snap.GetResources(resType)
		if len(resources) == 0 {
			log.Println("  (No resources of this type)")
			continue
		}

		for name, res := range resources {
			yamlBytes, err := yaml.Marshal(res)
			if err != nil {
				log.Printf("Error marshalling resource %s: %v\n", name, err)
				continue
			}

			log.Printf("--- Resource Name: %s ---\n%s\n", name, string(yamlBytes))
		}
	}
	log.Println("--- End of Snapshot Debug ---")
}
