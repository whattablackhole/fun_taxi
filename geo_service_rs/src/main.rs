mod consumers;
mod controllers;
mod models;
mod protos;
mod repositories;
mod services;
mod traits;
use std::env;

use crate::{
    consumers::trips::spawn_trip_geo_commands_consumer,
    protos::{
        gps::drivers_finder_service_server::DriversFinderServiceServer,
        trips::trips_finder_service_server::TripsFinderServiceServer,
    },
    services::{drivers_finder::DriversFinderServiceImpl, trips_finder::TripsFinderServiceImpl},
};
use consumers::geoposition::spawn_long_running_kafka_processor;
use redis::Client;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> () {
    env_logger::init();

    let kafka_handle: tokio::task::JoinHandle<Result<(), rdkafka::error::KafkaError>> =
        spawn_long_running_kafka_processor(
            env::var("KAFKA_BROKERS").expect("Resolving KAFKA_BROKERS var failed"),
            "geo_position_service_group".to_string(),
            vec!["gps_driver_position".to_string()],
        )
        .await;

    let message_bus_handle = spawn_trip_geo_commands_consumer().await;

    let client =
        Client::open(env::var("REDIS_ADDRESS").expect("REDIS_ADDRESS var resolving failed"))
            .expect("redis client creation failed");

    let redis: redis::aio::MultiplexedConnection = client
        .get_multiplexed_async_connection()
        .await
        .expect("redis multiplexed connection creation failed");

    let grpc_handle = Server::builder()
        .add_service(TripsFinderServiceServer::new(TripsFinderServiceImpl::new(
            redis.clone(),
        )))
        .add_service(DriversFinderServiceServer::new(
            DriversFinderServiceImpl::new(redis),
        ))
        .serve(env::var("GRPC_SERVER_ADDRESS").unwrap().parse().unwrap());

    tokio::select! {
        result = kafka_handle => {
            match result {
                Ok(Ok(())) => println!("Kafka finished successfully"),
                Ok(Err(e)) => eprintln!("Kafka returned an error: {:?}", e),
                Err(e) => eprintln!("Kafka task panicked or was cancelled: {:?}", e),
            }
        }

        result = message_bus_handle => {
            match result {
                Ok(Ok(())) => println!("Message bus finished successfully"),
                Ok(Err(e)) => eprintln!("Message bus returned an error: {:?}", e),
                Err(e) => eprintln!("Message bus task panicked or was cancelled: {:?}", e),
            }
        }

        result = grpc_handle => {
            match result {
                Ok(()) => println!("gRPC finished successfully"),
                Err(e) => eprintln!("gRPC task failed: {:?}", e),
            }
        }
    }
}
