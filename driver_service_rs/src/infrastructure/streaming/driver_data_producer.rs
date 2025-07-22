use crate::domain::position::GeoPosition;
use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use serde::Serialize;
use serde_json::ser;
use uuid::Uuid;

pub struct DriverDataProducer {
    producer: FutureProducer,
    topic_name: String,
}

#[derive(Serialize)]
pub struct DriverPositionPayload {
    driver_id: Uuid,
    position: GeoPosition,
    direction: i32,
}

impl DriverDataProducer {
    pub fn new(brokers: &str) -> Self {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            // TODO: make it for dev only
            .set("allow.auto.create.topics", "true")
            .create()
            .expect("Producer creation error");
        return DriverDataProducer {
            producer: producer,
            // TODO: move out into app consts
            topic_name: "t_driver_current_position".to_string(),
        };
    }

    pub async fn send_driver_position(&self, position_payload: DriverPositionPayload) {
        let payload = &ser::to_string(&position_payload).unwrap();
        let _ = self.producer
            .send(
                FutureRecord::to(&self.topic_name)
                    .payload(payload)
                    .key(&position_payload.driver_id.to_string())
                    .headers(
                        OwnedHeaders::new()
                            // TODO: move out into app consts
                            .insert(Header {
                                key: "message_type",
                                value: Some("driver_position"),
                            })
                            .insert(Header {
                                key: "schema_version",
                                value: Some("1.0"),
                            })
                            .insert(Header {
                                key: "producer",
                                value: Some("driver_service"),
                            }),
                    ),
                Timeout::Never,
            )
            .await;
    }
}
