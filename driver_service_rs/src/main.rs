use std::env;
use std::sync::Arc;

use crate::message_bus::trip_state_bus::spawn_trip_state_consumer;
use crate::web_api::controllers::driver_controller::DriverController;
use actix_web::http::StatusCode;
use actix_web::{App, HttpServer, web};
use actix_ws::Session;
use lapin::{Connection, ConnectionProperties};
use log::{error, info};
use tokio::sync::Mutex;
use tonic::transport::{Channel, Endpoint};

pub mod domain;
pub mod infrastructure;
pub mod message_bus;
pub mod proto;
pub mod traits;
pub mod web_api;

pub struct AppState {
    driver_session: Mutex<Option<Session>>,
    grpc_channel: Channel,
    bus_channel: lapin::Channel
}

impl AppState {
    async fn new(bus_channel: lapin::Channel) -> Self {
        Self {
            driver_session: Mutex::new(None),
            // driver_data_producer: Some(DriverDataProducer::new("localhost:9092")),
            grpc_channel: {
                let endpoint =
                    Endpoint::from_shared(env::var("GEO_SERVICE_GRPC_ADDRESS").unwrap()).unwrap();
                let channel = endpoint.connect_lazy();
                channel
            },
            bus_channel
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let rabbit_url = env::var("RABBIT_MQ").unwrap();
    let connection = Connection::connect(&rabbit_url, ConnectionProperties::default()).await.unwrap();
    let channel = connection.create_channel().await.unwrap();

    let app_state = web::Data::new(Arc::new(AppState::new(channel.clone()).await));
    let address = env::var("SERVER_IP_ADDRESS").unwrap();
    let port = u16::from_str_radix(&env::var("SERVER_PORT").unwrap(), 10).unwrap();

    println!("Starting server on {} {}", address, port);

    let web_server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route(
                "/",
                web::get().to(DriverController::connect_with_bot_driver),
            )
            .route(
                "/get_available_trips",
                web::post().to(DriverController::get_available_trips),
            )
            .route(
                "/accept_trip",
                web::post().to(DriverController::accept_trip),
            )
            .route(
                "stop_driver",
                web::post().to(|state: web::Data<Arc<AppState>>| async move {
                    let response = actix_web::HttpResponse::new(StatusCode::OK);
                    let mut session = state.driver_session.lock().await;

                    let cur = session.as_mut().unwrap();
                    cur.text("stop").await.unwrap();

                    response
                }),
            )
        // .route(
        //     "start_driver",
        //     web::post().to(|state| async move {

        //     })
        // )
    })
    .bind((address, port))?
    .run();

    let trip_state_bus = spawn_trip_state_consumer(channel.clone());

    tokio::select! {
        trip_state_bus_result = trip_state_bus => {
            match trip_state_bus_result {
                Ok(_)=> {
                    info!("grpc server finished without errors");
                },
                Err(e)=>{
                    error!("grpc server finished with error: {:?}", e);
                }
            }
        },
        web_server_result = web_server => {
            match web_server_result {
                Ok(_)=> {
                    info!("web server finished without errors");
                },
                Err(e)=>{
                    error!("web server finished with error: {:?}", e);
                }
            }
        }

    }

    Ok(())
}
