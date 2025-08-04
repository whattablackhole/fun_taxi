use std::{env, error::Error};

use crate::protos::trips::TripGeoPositionAddCommand;
use futures_util::StreamExt;
use lapin::{
    Connection, ConnectionProperties, ExchangeKind,
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicRejectOptions, ExchangeDeclareOptions,
        QueueBindOptions, QueueDeclareOptions,
    },
    types::FieldTable,
};
use log::{debug, error};
use redis::{Client, aio::MultiplexedConnection};

pub async fn spawn_trips_channel()
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
            .exchange_declare(
                "FunTaxi.Messages.Trips.V1:TripGeoPositionAddCommand",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;
        channel
            .queue_bind(
                "TripGeoCommands",
                "FunTaxi.Messages.Trips.V1:TripGeoPositionAddCommand",
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

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
            let delivery = delivery?;
            match delivery.exchange.as_str() {
                "FunTaxi.Messages.Trips.V1:TripGeoPositionAddCommand" => {
                    let command: TripGeoPositionAddCommand =
                        serde_json::from_slice(&delivery.data)?;

                    match add_new_trip(&command, &mut redis).await {
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

async fn add_new_trip(
    cmd: &TripGeoPositionAddCommand,
    connection: &mut MultiplexedConnection,
) -> redis::RedisResult<()> {
    let mut pipe = redis::pipe();
    pipe.atomic()
        .cmd("GEOADD")
        .arg("available_trips:start")
        .arg(&cmd.start_lat)
        .arg(&cmd.start_lon)
        .arg(&cmd.id)
        .cmd("GEOADD")
        .arg("available_trips:end")
        .arg(&cmd.end_lat)
        .arg(&cmd.end_lon)
        .arg(&cmd.id);

    pipe.query_async::<()>(connection).await?;
    Ok(())
}
