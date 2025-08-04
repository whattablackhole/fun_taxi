use std::{env, error::Error};

use crate::{
    repositories::trip_repository::TripRepository, traits::lapin::DeclareAndBindFanoutExchange,
};
use futures_util::StreamExt;
use lapin::{
    Connection, ConnectionProperties,
    options::{BasicAckOptions, BasicConsumeOptions, BasicRejectOptions, QueueDeclareOptions},
    types::FieldTable,
};
use log::{debug, error};
use redis::Client;

pub async fn spawn_trip_geo_commands_consumer()
-> tokio::task::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>> {
    return tokio::spawn(async {
        let client = Client::open(env::var("REDIS_ADDRESS")?)?;
        let mut redis = client.get_multiplexed_async_connection().await?;

        let rabbit_url = env::var("RABBIT_MQ")?;
        let connection = Connection::connect(&rabbit_url, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;

        channel
            .queue_declare(
                "TripGeoCommands",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        channel
            .declare_and_bind_fanout_exchange(
                "TripGeoCommands",
                "FunTaxi.Messages.Trips.V1:TripGeoPositionAddCommand",
            )
            .await
            .unwrap();
        channel
            .declare_and_bind_fanout_exchange(
                "TripGeoCommands",
                "FunTaxi.Messages.Trips.V1:TripGeoPositionRemoveCommand",
            )
            .await
            .unwrap();

        let mut consumer = channel
            .basic_consume(
                "TripGeoCommands",
                "geo_service",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        while let Some(delivery) = consumer.next().await {
            debug!("Start processing new delivery...");
            // NOTE:
            // Let thread fail and reload server until recovery logic is written
            let delivery = delivery?;
            match delivery.exchange.as_str() {
                "FunTaxi.Messages.Trips.V1:TripGeoPositionAddCommand" => {
                    match serde_json::from_slice(&delivery.data) {
                        Ok(cmd) => match TripRepository::add_new_trip(&cmd, &mut redis).await {
                            Ok(_) => {
                                delivery.ack(BasicAckOptions::default()).await?;
                                debug!("Successfully added driver_position to Redis");
                            }
                            Err(e) => {
                                error!("Redis geo_add failed: {:?}", e);
                                delivery
                                    .reject(BasicRejectOptions { requeue: false })
                                    .await?;
                            }
                        },
                        Err(err) => {
                            error!("Failed to parse TripGeoPositionAddCommand: {:?}", err);
                            delivery
                                .reject(BasicRejectOptions { requeue: false })
                                .await?;
                        }
                    }
                }
                "FunTaxi.Messages.Trips.V1:TripGeoPositionRemoveCommand" => {
                    match serde_json::from_slice(&delivery.data) {
                        Ok(cmd) => match TripRepository::remove_trip(&cmd, &mut redis).await {
                            Ok(_) => {
                                delivery.ack(BasicAckOptions::default()).await?;
                                debug!("Successfully added driver_position to Redis");
                            }
                            Err(e) => {
                                error!("Trip geo index removal from DB failed: {:?}", e);
                                delivery
                                    .reject(BasicRejectOptions { requeue: false })
                                    .await?;
                            }
                        },
                        Err(err) => {
                            error!("Failed to parse TripGeoPositionRemoveCommand: {:?}", err);
                            delivery
                                .reject(BasicRejectOptions { requeue: false })
                                .await?;
                        }
                    }
                }
                name => {
                    error!("Exchange name resolving failed: {:?}", name);
                    delivery
                        .reject(BasicRejectOptions { requeue: false })
                        .await?;
                }
            }
        }

        Ok(())
    });
}
