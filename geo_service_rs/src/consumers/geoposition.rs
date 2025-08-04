use std::env;

use log::{debug, error, info, warn};
use prost::Message as ProstMessage;
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{
    stream_consumer::StreamConsumer, BaseConsumer, Consumer, ConsumerContext, Rebalance,
};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::message::{Message, ToBytes};
use rdkafka::topic_partition_list::TopicPartitionList;
use redis::geo::Coord;
use redis::{Client, RedisResult};
use schema_registry_converter::async_impl::proto_raw::ProtoRawDecoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;

use crate::protos::gps::UpdateDriverPositionPayload;
use redis::AsyncCommands;
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}
type GeopositionConsumer = StreamConsumer<CustomContext>;
pub async fn spawn_long_running_kafka_processor(
    brokers: String,
    group_id: String,
    topics: Vec<String>,
) -> tokio::task::JoinHandle<Result<(), KafkaError>> {
    let handle = tokio::spawn(async move {
        let client =
            Client::open(env::var("REDIS_ADDRESS").expect("REDIS_ADDRESS var resolving failed"))
                .expect("redis client creation failed");
        let mut redis: redis::aio::MultiplexedConnection = client
            .get_multiplexed_async_connection()
            .await
            .expect("redis multiplexed connection creation failed");
            
        let context: CustomContext = CustomContext;
        let topics_str: Vec<&str> = topics.iter().map(|w| w.as_str()).collect();
        let decoder = ProtoRawDecoder::new(SrSettings::new(String::from(
            env::var("SCHEMA_REGISTRY_URL").expect("SCHEMA_REGISTRY_URL var resolving failed"),
        )));
        let consumer: GeopositionConsumer = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            //.set("statistics.interval.ms", "30000")
            //.set("auto.offset.reset", "smallest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .expect("Consumer creation failed");
        consumer
            .subscribe(&topics_str)
            .expect("Can't subscribe to specified topics");

        loop {
            match consumer.recv().await {
                Err(e) => return Err(e),
                Ok(m) => match decoder.decode(m.payload()).await {
                    Ok(result_option) => match result_option {
                        Some(result) => {
                            debug!("Received new message from kafka {:?}", result.full_name);
                            if let Ok(payload) =
                                UpdateDriverPositionPayload::decode(result.bytes.to_bytes())
                            {
                                debug!(
                                    "Uploading driver_position to redis {:?}",
                                    payload.driver_id
                                );

                                let location =
                                    payload.location.as_ref().expect("location retrival failed");
                                let result: RedisResult<isize> = redis
                                    .geo_add(
                                        "drivers_position",
                                        (
                                            Coord::lon_lat(location.longitude, location.latitude),
                                            payload.driver_id,
                                        ),
                                    )
                                    .await;
                                if let Err(e) = result {
                                    error!("Redis geo_add failed: {:?}", e);
                                } else {
                                    debug!("Successfully added driver_position to Redis");
                                }
                            } else {
                                info!("Unable to decode payload");
                            }
                        }
                        None => {
                            info!("Empty deconding result");
                        }
                    },
                    Err(e) => warn!("error getting payload: {}", e),
                },
            };
        }
    });
    return handle;
}
