mod consumers;
mod controllers;
mod models;
mod services;
mod tools;
mod protos;
use std::env;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use consumers::geoposition::spawn_long_running_kafka_processor;
use dotenv::from_filename;

struct AppState {
    pub _app_name: String,
}

#[tokio::main]
async fn main() -> () {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    load_environment();

    let address = env::var("SERVER_IP_ADDRESS").expect("Resolving SERVER_IP_ADDRESS var failed");
    let port = u16::from_str_radix(
        &env::var("SERVER_PORT").expect("Resolving SERVER_PORT var failed"),
        10,
    )
    .expect("Converting SERVER_PORT into u16 failed");
    let brokers = env::var("KAFKA_BROKERS").expect("Resolving KAFKA_BROKERS var failed");

    let kafka_handle = spawn_long_running_kafka_processor(
        brokers,
        "geo_position_service_group".to_string(),
        vec!["gps_driver_position".to_string()],
    ).await;

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(AppState {
                _app_name: String::from("Taxi"),
            }))
            .configure(controllers::navigation::config)
    })
    .bind((address, port))
    .expect("Binding address and port failed")
    .run();

    tokio::select! {
        _ = kafka_handle => {
            println!("Kafka processor finished, exiting");
        }
        _ = server => {
            println!("HTTP server finished, exiting");
        }
    }
}

fn load_environment() {
    let env = env::var("APP_ENV").unwrap_or_else(|_| "dev".into());
    let filename = format!(".env.{}", env);
    from_filename(&filename).ok();
}
