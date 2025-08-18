pub mod infrastructure;
pub mod services;
pub mod traits;
pub mod web_api;

use crate::{
    infrastructure::websockets::ws_connection_manager::WebSocketConnectionManager,
    services::{
        msg_service::StreamGateMessageServiceImpl,
        streamgate::stream_gate_message_service_server::StreamGateMessageServiceServer,
    },
    web_api::controllers::user_controller::{ws_handler},
};
use axum::{
    Router,
    routing::{any, get},
};
use axum_tonic::NestTonic;
use dotenv::from_filename;
use hyper::server::conn::http2;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    service::TowerToHyperService,
};
use log::{error, info};
use redis::Client;
use std::{collections::HashMap, env, fs::File, io::BufReader, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tower::Service;

use rustls::crypto::aws_lc_rs::default_provider;
use rustls::{crypto::CryptoProvider, pki_types::CertificateDer};
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::ServerConfig;


pub struct AppState {
    pub ws_manager: Arc<WebSocketConnectionManager>,
}

pub fn load_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let cert_file = File::open("streamgate-server.crt")?;
    let mut cert_reader = BufReader::new(cert_file);

    let certs: Vec<CertificateDer<'static>> =
        rustls_pemfile::certs(&mut cert_reader).collect::<Result<Vec<_>, _>>()?;

    let key_file = File::open("streamgate-server.key")?;
    let mut key_reader = BufReader::new(key_file);

    let key_der = rustls_pemfile::private_key(&mut key_reader)?
        .ok_or("Could not find private key in file")?;
    let config = ServerConfig::builder()
        .with_no_client_auth() // Use this for standard TLS
        .with_single_cert(certs, key_der)?;

    Ok(config)
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    CryptoProvider::install_default(default_provider()).unwrap();

    load_environment();
    env_logger::init();

    let config = load_config().unwrap();

    let acceptor = TlsAcceptor::from(Arc::new(config));

    let redis = {
        let client =
            Client::open(env::var("REDIS_ADDRESS").expect("REDIS_ADDRESS var resolving failed"))
                .expect("redis client creation failed");

        let redis: redis::aio::MultiplexedConnection = client
            .get_multiplexed_async_connection()
            .await
            .expect("redis multiplexed connection creation failed");
        redis
    };

    let address = env::var("SERVER_IP_ADDRESS").unwrap();
    let port = u16::from_str_radix(&env::var("SERVER_PORT").unwrap(), 10).unwrap();


    let m = Arc::new(WebSocketConnectionManager {
        sessions: Mutex::new(HashMap::new()),
        hostname: env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string()),
        redis: redis.clone(),
    });
    let rest_router = Router::new()
        .route("/ws", any(ws_handler))
        .route("/health", get(async || "Ok"))
        .with_state(Arc::new(AppState {
            ws_manager: m.clone(),
        }));

    let grpc_router = Router::new().nest_tonic(StreamGateMessageServiceServer::new(
        StreamGateMessageServiceImpl {
            ws_manager: m.clone(),
        },
    ));

    let router = rest_router.merge(grpc_router);

    let service = router.into_make_service_with_connect_info::<SocketAddr>();

    let listener = TcpListener::bind(format!("{}:{}", address, port))
        .await
        .unwrap();

    info!("Serving streamgate on: ip: {}, port: {}", address, port);

    loop {
        let (socket, remote_addr) = listener.accept().await.unwrap();
        let acceptor = acceptor.clone();
        let tower_service = service.clone().call(remote_addr).await.unwrap();

        tokio::spawn(async move {
            let hyper_service_for_connection = TowerToHyperService::new(tower_service);

            if let Err(e) = async {
                let tls_stream = acceptor.accept(socket).await?;
                let io = TokioIo::new(tls_stream);
                match io.inner().get_ref().1.alpn_protocol() {
                    Some(b"h2") => {
                        let builder: http2::Builder<TokioExecutor> =
                            hyper::server::conn::http2::Builder::new(TokioExecutor::new());
                        let conn = builder.serve_connection(io, hyper_service_for_connection);
                        conn.await?;
                    }
                    _ => {
                        let conn = hyper::server::conn::http1::Builder::new()
                            .preserve_header_case(true)
                            .serve_connection(io, hyper_service_for_connection);

                        conn.with_upgrades().await?;
                    }
                }
                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
            }
            .await
            {
                error!("Connection error: {}", e);
            }
        });
    }
}

fn load_environment() {
    let env = env::var("APP_ENV").unwrap_or_else(|_| "dev".into());
    let filename = format!(".env.{}", env);
    from_filename(&filename).ok();
}
