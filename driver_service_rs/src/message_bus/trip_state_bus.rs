use std::{env, error::Error};

use crate::proto::fun_taxi_grpc_streamgate::{
    StreamGateMessage, stream_gate_message_service_client::StreamGateMessageServiceClient,
};

use crate::traits::lapin::DeclareAndBindFanoutExchange;
use futures_util::StreamExt;
use lapin::Channel;
use lapin::{
    Connection, ConnectionProperties,
    options::{BasicAckOptions, BasicConsumeOptions, BasicRejectOptions, QueueDeclareOptions},
    types::FieldTable,
};
use log::{debug, error, info};
use tonic::Request;

pub fn spawn_trip_state_consumer(
    bus_channel: Channel,
) -> tokio::task::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>> {
    return tokio::spawn(async move {
        let grpc_channel =
            tonic::transport::Channel::from_shared(env::var("STREAMGATE_GRPC_ADDRESS")?)?
                .connect_lazy();

        bus_channel
            .queue_declare(
                "Q-DriverTripState",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        bus_channel
            .declare_and_bind_fanout_exchange(
                "Q-DriverTripState",
                "FunTaxi.Messages.Trips.V1:TripAssignedToDriver",
            )
            .await
            .unwrap();

        let mut consumer = bus_channel
            .basic_consume(
                "Q-DriverTripState",
                "driver_service",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        while let Some(delivery) = consumer.next().await {
            debug!("Received msg from Q-DriverTripState queue");
            // // NOTE:
            // // Let thread fail and reload server until recovery logic is written
            let delivery = delivery?;

            match delivery.exchange.as_str() {
                "FunTaxi.Messages.Trips.V1:TripAssignedToDriver" => {
                    debug!("TripAssignedToDriver message matched");
                    let headers = match delivery.properties.headers().as_ref() {
                        Some(headers) => headers,
                        None => {
                            debug!("No headers found, rejecting...");
                            delivery
                                .reject(BasicRejectOptions { requeue: false })
                                .await?;
                            continue;
                        }
                    };

                    let id_amq_value = match headers.inner().get("x-user-id") {
                        Some(value) => value,
                        None => {
                            debug!("No x-user-id header found, rejecting...");
                            delivery
                                .reject(BasicRejectOptions { requeue: false })
                                .await?;
                            continue;
                        }
                    };

                    let id_string = match id_amq_value.as_long_string() {
                        Some(s) => s,
                        None => {
                            debug!("Can't parse x-user-id as long string, rejecting...");
                            delivery
                                .reject(BasicRejectOptions { requeue: false })
                                .await?;
                            continue;
                        }
                    };

                    let mut streamgate_client =
                        StreamGateMessageServiceClient::new(grpc_channel.clone());

                    let message = StreamGateMessage {
                        payload: delivery.data.clone(),
                        user_id: id_string.to_string(),
                    };

                    let mut request = Request::new(message);
                    request
                        .metadata_mut()
                        .insert("x-user-id", id_string.to_string().parse().unwrap());
                    debug!("Sending message to stream gate...");

                    match streamgate_client.send_message(request).await {
                        Ok(r) => {
                            delivery.ack(BasicAckOptions::default()).await?;
                        }
                        Err(e) => {
                            debug!("Sending message to stream gate failed..., {:?}", e);
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
