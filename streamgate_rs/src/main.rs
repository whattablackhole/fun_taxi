use crate::web_api::controllers::user_controller::UserController;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_ws::Session;
use dotenv::from_filename;
use log::info;
use redis::{Client, aio::MultiplexedConnection};
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;

pub mod infrastructure;
pub mod traits;
pub mod web_api;

pub struct AppState {
    sessions: Mutex<HashMap<uuid::Uuid, Session>>,
    hostname: String,
    redis: MultiplexedConnection,
}
impl AppState {
    async fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            hostname: env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string()),

            redis: {
                let client = Client::open(
                    env::var("REDIS_ADDRESS").expect("REDIS_ADDRESS var resolving failed"),
                )
                .expect("redis client creation failed");

                let redis: redis::aio::MultiplexedConnection = client
                    .get_multiplexed_async_connection()
                    .await
                    .expect("redis multiplexed connection creation failed");
                redis
            },
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");
    load_environment();
    env_logger::init();

    let state = Arc::new(AppState::new().await);
    let app_state = web::Data::new(state.clone());
    let address = env::var("SERVER_IP_ADDRESS").unwrap();
    let port = u16::from_str_radix(&env::var("SERVER_PORT").unwrap(), 10).unwrap();
    info!("Serving streamgate on: ip: {}, port: {}", address, port);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(UserController::connect_ws))
            .route(
                "/health",
                web::get().to(async |req: HttpRequest| {
                    let port = u16::from_str_radix(&env::var("SERVER_PORT").unwrap(), 10).unwrap();
                    println!("Health request came to port: {}", port);
                    HttpResponse::Ok().body("OK")
                }),
            )
    })
    .bind((address, port))?
    .run()
    .await;

    Ok(())
}

fn load_environment() {
    let env = env::var("APP_ENV").unwrap_or_else(|_| "dev".into());
    let filename = format!(".env.{}", env);
    from_filename(&filename).ok();
}
